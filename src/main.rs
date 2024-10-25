use clap::Parser;
use tracing::{warn, Level};
use tracing_subscriber::FmtSubscriber;

mod cli;

use cli::{Cli, Commands, PipelineCommands};

fn configure_logging() {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            // .json()
            .finish(),
    )
    .expect("Setting default subscriber failed");
}

#[tokio::main]
async fn main() {
    configure_logging();

    let cli = Cli::parse();

    let pipeline_commands = match &cli.command {
        Some(Commands::Pipelines { command }) => Some(command),
        _ => {
            warn!("Please specify a subcommand to call!");
            None
        }
    };

    match pipeline_commands {
        Some(&PipelineCommands::Create { update }) => {
            cli.create_pipelines(update).await;
        }

        Some(&PipelineCommands::Update) => {
            cli.update_pipelines().await;
        }

        Some(&PipelineCommands::Validate) => {
            cli.validate_pipelines().await;
        }

        Some(&PipelineCommands::Deploy) => {
            cli.deploy_pipelines().await;
        }

        None => {
            warn!("Please specify a subcommand to call!")
        }
    }
}
