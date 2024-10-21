use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;

use deepset_cloud_api::types::sdk::AccessTokenAuth;
use deepset_cloud_api::{DeepsetCloudApi, DeepsetCloudSettings};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets the directory where the pipeline artifacts are stored. Defaults to `./pipelines`.
    #[arg(short, long, value_name = "DIR")]
    pub pipeline_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Pipeline related subcommands
    Pipelines {
        #[command(subcommand)]
        command: PipelineCommands,
    },
}

#[derive(Subcommand)]
pub enum PipelineCommands {
    /// Create new pipelines from the artifacts stored in `pipeline_dir`
    Create {
        /// When set to `true` will update the pipeline if it already exist
        #[arg[short, long]]
        update: bool,
    },

    /// Update existing pipelines with the artifacts stored in `pipeline_dir`
    Update,

    /// Validate all pipelines whose artifacts are in the `pipeline_dir`
    Validate,

    /// Deploy all the pipelines in the `pipeline_dir`
    Deploy,
}

impl Cli {
    #[allow(dead_code)]
    fn settings(&self) -> DeepsetCloudSettings {
        DeepsetCloudSettings::init()
            .expect("Could not determine settings from environment variables!")
    }

    #[allow(dead_code)]
    fn deepset_cloud_api(&self) -> DeepsetCloudApi {
        let settings = self.settings();
        DeepsetCloudApi::builder()
            .with_authenticator(AccessTokenAuth::new(settings.api_key))
            .build()
    }

    #[allow(dead_code)]
    fn path(&self) -> PathBuf {
        self.pipeline_dir
            .clone()
            .unwrap_or(PathBuf::from("./pipelines"))
    }

    pub fn create_pipelines(&self, _update: bool) {
        info!("Creating pipelines with...");
    }

    pub fn update_pipelines(&self) {
        info!("Updating pipelines...");
    }

    pub fn validate_pipelines(&self) {
        info!("Validating pipelines...");
    }

    pub fn deploy_pipelines(&self) {
        info!("Deploying pipelines...");
    }
}
