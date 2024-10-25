use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process;
use tracing::{error, info, warn};

use deepset_cloud_api::types::sdk::AccessTokenAuth;
use deepset_cloud_api::types::PipelineIn;
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
        #[arg[short, long, action]]
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

    fn path(&self) -> PathBuf {
        self.pipeline_dir
            .clone()
            .unwrap_or(PathBuf::from("./pipelines"))
            .canonicalize()
            .unwrap()
    }

    fn into_pipeline(&self, path: &Path) -> Option<PipelineIn> {
        let query_yaml = path.join("query.yaml");
        let indexing_yaml = path.join("indexing.yaml");

        if !(query_yaml.is_file() && indexing_yaml.is_file()) {
            warn!(
                "Both indexing.yaml and query.yaml must exist in {}",
                path.display()
            );
            return None;
        }

        let pipeline_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        let indexing_yaml_content = std::fs::read_to_string(&indexing_yaml)
            .map_err(|e| warn!("Failed to read indexing.yaml: {}", e))
            .ok()?;

        let query_yaml_content = std::fs::read_to_string(&query_yaml)
            .map_err(|e| warn!("Failed to read query.yaml: {}", e))
            .ok()?;

        Some(
            PipelineIn::builder()
                .name(pipeline_name)
                .indexing_yaml(&indexing_yaml_content)
                .query_yaml(&query_yaml_content)
                .build(),
        )
    }

    fn load_pipelines(&self, path: PathBuf) -> Vec<PipelineIn> {
        let mut pipeline_dirs = Vec::new();

        match std::fs::read_dir(&path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                            pipeline_dirs.push(entry.path());
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to read directory {}: {}", path.display(), e);
                process::exit(1);
            }
        }

        let mut pipelines = Vec::new();
        for dir in pipeline_dirs {
            match self.into_pipeline(dir.as_path()) {
                Some(pipeline) => pipelines.push(pipeline),
                None => {
                    error!("Failed to load pipeline in directory: {}", dir.display());
                    process::exit(1);
                }
            }
        }
        return pipelines;
    }

    pub fn create_pipelines(&self, _update: bool) {
        info!("Creating pipelines...");
        let path = self.path();
        let pipelines = self.load_pipelines(path);
        dbg!(pipelines);
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
