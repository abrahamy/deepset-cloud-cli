use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod pipeline;

pub use self::pipeline::*;

pub mod sdk {
    pub use apisdk::*;
}

#[derive(Debug, Deserialize, Serialize, Getters)]
pub struct User {
    user_id: Uuid,
    given_name: String,
    family_name: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum ServiceLevel {
    DRAFT,
    #[default]
    DEVELOPMENT,
    PRODUCTION,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeepsetCloudVersion {
    V1,
    #[default]
    V2,
}

#[derive(Debug, Deserialize, Serialize, Getters)]
pub struct Workspace {
    name: String,
    workspace_id: Uuid,
    languages: Vec<String>,
    default_idle_timeout_in_seconds: u64,
}
