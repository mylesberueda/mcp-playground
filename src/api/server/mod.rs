mod calculator;
use calculator::*;

use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

pub(crate) struct McpServer {}

impl McpServer {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn start(self, port: u16) -> color_eyre::Result<()> {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "debug".to_string().into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();

        let config = rmcp::transport::sse_server::SseServerConfig {
            bind: format!("localhost:{port}").parse()?,
            sse_path: "/sse".to_string(),
            post_path: "/message".to_string(),
            ct: tokio_util::sync::CancellationToken::new(),
            sse_keep_alive: None,
        };

        let (sse_server, router) = rmcp::transport::sse_server::SseServer::new(config);

        let listener = tokio::net::TcpListener::bind(sse_server.config.bind).await?;

        let ct = sse_server.config.ct.child_token();

        let server = axum::serve(listener, router).with_graceful_shutdown(async move {
            ct.cancelled().await;
            tracing::info!("exiting server");
        });

        tokio::spawn(async move {
            if let Err(e) = server.await {
                tracing::error!(error = %e, "shutting down server with error")
            }
        });

        let ct = sse_server.with_service(Calculator::new);

        tokio::signal::ctrl_c().await?;
        ct.cancel();
        Ok(())
    }
}
