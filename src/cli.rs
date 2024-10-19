use clap::{Parser, Subcommand};
use std::path::PathBuf;

use deepset_cloud_api::types::sdk::AccessTokenAuth;
use deepset_cloud_api::{DeepsetCloudApi, DeepsetCloudSettings};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Checks if the a workspace with the configured workspace name exists or not
    CheckIfWorkspaceExists,

    /// Create or update pipelines
    CreateOrUpdatePipelines {
        /// Sets the directory where the pipeline artifacts are stored. Defaults to `./pipelines`.
        #[arg(short, long, value_name = "DIR")]
        pipeline_dir: Option<PathBuf>,
    },

    /// Validate pipelines
    ValidatePipelines {
        /// Sets the directory where the pipeline artifacts are stored. Defaults to `./pipelines`.
        #[arg(short, long, value_name = "DIR")]
        pipeline_dir: Option<PathBuf>,
    },

    /// Deploy pipelines to production workspace
    DeployPipelines {
        /// Sets the directory where the pipeline artifacts are stored. Defaults to `./pipelines`.
        #[arg(short, long, value_name = "DIR")]
        pipeline_dir: Option<PathBuf>,
    },
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
}
