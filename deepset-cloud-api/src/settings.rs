use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeepsetCloudSettings {
    pub api_key: String,
    pub workspace_name: String,
}

impl DeepsetCloudSettings {
    pub fn init() -> Result<Self, ConfigError> {
        Config::builder()
            .set_default("workspace_name", "default")?
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
