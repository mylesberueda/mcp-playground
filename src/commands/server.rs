#[derive(clap::Args)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Option<Commands>,
    args: Option<String>,
}

#[derive(clap::Subcommand)]
pub(crate) enum Commands {
    Start { port: Option<u16> },
}

pub(crate) async fn run(args: &Arguments) -> crate::Result<()> {
    match &args.command {
        Some(commands) => match commands {
            Commands::Start { port } => start_server(port).await,
        },
        None => example_fn(&None, &None),
    }
}

fn example_fn(_arg1: &Option<String>, _arg2: &Option<String>) -> crate::Result<()> {
    println!("This is an example function");
    Ok(())
}

async fn start_server(port: &Option<u16>) -> crate::Result<()> {
    crate::api::McpServer::new()
        .start(port.unwrap_or(8080))
        .await
}
