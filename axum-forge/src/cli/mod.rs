pub mod migration;

use clap::{Parser, Subcommand};

use crate::error::Result;

#[derive(Debug, Parser)]
#[command(name = "axum-forge")]
#[command(version)]
#[command(about = "Axum Forge CLI")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Command::Migration(command) => command.run().await,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Migration(migration::MigrationCommand),
}