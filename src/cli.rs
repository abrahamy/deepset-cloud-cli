use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process;
use tracing::{error, info, warn};

use deepset_cloud_api::types::sdk::AccessTokenAuth;
use deepset_cloud_api::types::{DeepsetCloudVersion, PipelineIn};
use deepset_cloud_api::DeepsetCloudSettings;

#[cfg(not(debug_assertions))]
use deepset_cloud_api::DeepsetCloudApi;

#[cfg(debug_assertions)]
use deepset_cloud_api::DeepsetCloudDevApi;

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

    #[cfg(not(debug_assertions))]
    fn deepset_cloud_api(&self, settings: &DeepsetCloudSettings) -> DeepsetCloudApi {
        DeepsetCloudApi::builder()
            .with_authenticator(AccessTokenAuth::new(&settings.api_key))
            .build()
    }

    #[cfg(debug_assertions)]
    fn deepset_cloud_api(&self, settings: &DeepsetCloudSettings) -> DeepsetCloudDevApi {
        DeepsetCloudDevApi::builder()
            .with_authenticator(AccessTokenAuth::new(&settings.api_key))
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

    pub async fn create_pipelines(&self, update: bool) {
        info!("Creating pipelines...");

        let path = self.path();
        let settings = self.settings();

        let api = self.deepset_cloud_api(&settings);

        let pipelines = self.load_pipelines(path);

        for payload in pipelines.iter() {
            let workspace_name = &settings.workspace_name;
            match api.create_pipeline(workspace_name, payload).await {
                Ok(_) => {
                    info!("Created pipeline {} successfully", payload.name());
                }
                Err(err) => {
                    if !(update && err.as_error_code() == 409) {
                        error!("Failed to create pipeline {}: {}", payload.name(), err);
                        process::exit(1);
                    }

                    api.update_pipeline_yaml(workspace_name, payload)
                        .await
                        .expect(&format!("Failed to update pipeline {}", payload.name()));
                    info!("Updated pipeline {} successfully", payload.name());
                }
            }
        }
    }

    pub async fn update_pipelines(&self) {
        info!("Updating pipelines...");

        let path = self.path();
        let settings = self.settings();

        let api = self.deepset_cloud_api(&settings);

        let pipelines = self.load_pipelines(path);

        for payload in pipelines.iter() {
            api.update_pipeline_yaml(&settings.workspace_name, payload)
                .await
                .expect(&format!("Failed to update pipeline {}", payload.name()));

            info!("Updated pipeline {} successfully", payload.name());
        }
    }

    pub async fn validate_pipelines(&self) {
        info!("Validating pipelines...");

        let path = self.path();

        let settings = self.settings();

        let api = self.deepset_cloud_api(&settings);

        api.validate_pipelines(&settings.workspace_name, &DeepsetCloudVersion::V2)
            .await
            .expect(&format!(
                "Validation failed for pipelines in {}",
                path.display()
            ));

        info!(
            "Validation for pipelines in {} was successful",
            path.display()
        );
    }

    pub async fn deploy_pipelines(&self) {
        info!("Deploying pipelines...");
    }
}
