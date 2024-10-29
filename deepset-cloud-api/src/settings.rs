use config::{Config, ConfigError, Environment};
use serde::Deserialize;
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct DeepsetCloudSettings {
    pub api_key: String,
    pub base_url: String,
    pub workspace_name: String,
}

impl DeepsetCloudSettings {
    #[instrument]
    pub fn init() -> Result<Self, ConfigError> {
        Config::builder()
            .set_default("workspace_name", "default")?
            .set_default("base_url", "https://api.cloud.deepset.ai/")?
            .add_source(
                Environment::default()
                    .prefix("deepset_cloud")
                    .prefix_separator("_")
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()
    }
}
