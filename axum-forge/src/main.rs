mod cli;
mod error;
mod migration;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(err) = cli.run().await {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}