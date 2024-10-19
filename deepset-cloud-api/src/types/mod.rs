use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod pipeline;

pub use self::pipeline::*;

pub mod sdk {
    pub use apisdk::*;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    user_id: Uuid,
    given_name: String,
    family_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceLevel {
    DRAFT,
    DEVELOPMENT,
    PRODUCTION,
}

impl Default for ServiceLevel {
    fn default() -> Self {
        ServiceLevel::DEVELOPMENT
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeepsetCloudVersion {
    V1,
    V2,
}

impl Default for DeepsetCloudVersion {
    fn default() -> Self {
        DeepsetCloudVersion::V2
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    name: String,
    workspace_id: Uuid,
    languages: Vec<String>,
    default_idle_timeout_in_seconds: u64,
}
