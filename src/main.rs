mod api;
mod commands;
use clap::Parser as _;

type Result<T> = color_eyre::Result<T>;

#[derive(clap::Parser)]
#[clap(name = "MCP Playground")]
#[clap(author = "Myles <myles@polypixel.io>")]
#[clap(version = "0.1.0")]
#[clap(about = "A simple project to play with mcp servers")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

const SCAFFOLD_ABOUT: &str = "
Scaffolding command for quickly generating new files in your project

This command will not show up in release builds and is only here for your 
convenience during development.";

#[derive(clap::Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    /// Basic command that does things and stuff
    Basic,
    Example(commands::example::Arguments),
    #[cfg(debug_assertions)]
    #[clap(arg_required_else_help = true)]
    #[clap(about = "Scaffolding command for quickly generating new files in your project")]
    #[clap(long_about = SCAFFOLD_ABOUT)]
    Scaffold(commands::scaffold::Arguments),
    Server(commands::server::Arguments),
}

#[tokio::main]
async fn main() -> crate::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    if let Some(cmds) = &cli.command {
        match cmds {
            Commands::Basic => basic_command(),
            Commands::Example(args) => commands::example::run(args),
            #[cfg(debug_assertions)]
            Commands::Scaffold(args) => commands::scaffold::run(args),
            Commands::Server(args) => commands::server::run(args).await,
        }?;
    };

    Ok(())
}

fn basic_command() -> crate::Result<()> {
    println!("Running the basic command from the top level");
    Ok(())
}
