use clap::{Parser, Subcommand};
use serde_json::Value;
use uuid::Uuid;

#[derive(Subcommand)]
enum Command {
    /// Start HARM's services.
    Start {
        #[clap(default_value = "10658", long, short)]
        /// The port HARM's API should run on.
        port: u16,

        #[clap(default_value = "sqlite::memory:", long, short = 'd')]
        /// Where HARM's sqlite database should be.
        database_url: String,

        #[clap(long, short = 'r')]
        /// Where Arma Reforger Server is installed
        reforger: String,
    },

    ExportConfig {
        #[clap(long, short)]
        id: Uuid,
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
        Command::Start {
            port,
            database_url,
            reforger,
        } => harm_api::start(*port, database_url.clone(), reforger.clone()).await,

        Command::ExportConfig { id } => {
            let resp = reqwest::get(format!("http://localhost:10658/servers/{}", id))
                .await
                .map_err(|e| format!("failed to query server: {}", e))?
                .json::<Value>()
                .await
                .map_err(|e| format!("failed to parse response: {}", e))?;

            let cfg = resp.get("config").unwrap().to_string();

            println!("{}", cfg);

            Ok(())
        }
    }
}
