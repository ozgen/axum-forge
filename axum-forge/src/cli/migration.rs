use clap::{Args, Subcommand};

use crate::error::Result;

#[derive(Debug, Args)]
pub struct MigrationCommand {
    #[command(subcommand)]
    command: MigrationSubcommand,
}

impl MigrationCommand {
    pub async fn run(self) -> Result<()> {
        match self.command {
            MigrationSubcommand::Add { name } => crate::migration::add(&name),
            MigrationSubcommand::Up { database_url } => crate::migration::up(database_url).await,
            MigrationSubcommand::Down { database_url } => crate::migration::down(database_url).await,
            MigrationSubcommand::Status { database_url } => {
                crate::migration::status(database_url).await
            }
        }
    }
}

#[derive(Debug, Subcommand)]
enum MigrationSubcommand {
    Add {
        name: String,
    },
    Up {
        #[arg(long)]
        database_url: Option<String>,
    },
    Down {
        #[arg(long)]
        database_url: Option<String>,
    },
    Status {
        #[arg(long)]
        database_url: Option<String>,
    },
}