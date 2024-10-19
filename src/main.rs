use clap::Parser;
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod cli;

use cli::{Cli, Commands};

fn configure_logging() {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .json()
            .finish(),
    )
    .expect("Setting default subscriber failed");
}

#[tokio::main]
async fn main() {
    configure_logging();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::CheckIfWorkspaceExists) => {
            info!("Checking if workspace exists...")
        }
        Some(Commands::CreateOrUpdatePipelines { pipeline_dir: _ }) => {
            info!("Creating or updating pipelines...");
        }
        Some(Commands::ValidatePipelines { pipeline_dir: _ }) => {
            info!("Validating pipelines...");
        }
        Some(Commands::DeployPipelines { pipeline_dir: _ }) => {}
        None => {
            warn!("Please specify a subcommand to call!")
        }
    }
}
