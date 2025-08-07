use crate::cli::{Cli, Commands};
use clap::Parser;
use shadow_rs::shadow;
use tracing::error;

mod api;
mod cli;
pub mod model;
mod provider;
mod server;
mod service;
mod website;

shadow!(build);
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    match start().await {
        Ok(_) => {}
        Err(e) => error!("{:#}", e),
    }
    Ok(())
}

async fn start() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command.unwrap_or_else(Default::default) {
        Commands::Run(args) => {
            server::run(args).await?;
        }
    }
    Ok(())
}
