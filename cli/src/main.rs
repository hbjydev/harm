use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    /// Start HARM's services.
    Start {
        #[clap(default_value = "10658", long)]
        port: u16,

        #[clap(default_value = "sqlite::memory:", long)]
        database_url: String,
    },
}

#[derive(Parser)]
#[command(author = "Hayden Young <hayden@hbjy.dev>")]
#[command(about = "An Arma Reforger server manager.")]
#[command(version, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let Cli { command } = cli;

    match &command {
        Command::Start { port, database_url } => {
            api::start(port.clone(), database_url.clone()).await
        }
    }
}
