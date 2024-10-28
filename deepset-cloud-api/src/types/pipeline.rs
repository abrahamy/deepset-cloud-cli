use chrono::{DateTime, Utc};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{DeepsetCloudVersion, ServiceLevel, User};

#[derive(Debug, Deserialize, Serialize, Getters)]
pub struct PipelineIndexingStatus {
    pending_file_count: u64,
    failed_file_count: u64,
}

#[derive(Debug, Deserialize, Serialize, Getters)]
pub struct Pipeline {
    name: String,
    pipeline_id: Uuid,
    status: String,
    desired_status: String,
    created_at: DateTime<Utc>,
    deleted: bool,
    is_default: bool,
    created_by: Option<User>,
    last_edited_by: Option<User>,
    last_edited_at: Option<DateTime<Utc>>,
    supports_prompt: bool,
    output_type: String,
    last_deployed_at: Option<DateTime<Utc>>,
    service_level: ServiceLevel,
    idle_timeout_in_seconds: Option<u64>,
    deepset_cloud_version: DeepsetCloudVersion,
    indexing: PipelineIndexingStatus,
}

#[derive(Debug, Default, Deserialize, Serialize, Getters)]
pub struct PipelineIn {
    name: String,
    config: Option<String>,
    indexing_yaml: Option<String>,
    query_yaml: Option<String>,
    deepset_cloud_version: DeepsetCloudVersion,
}

impl PipelineIn {
    pub fn builder() -> PipelineInBuilder {
        PipelineInBuilder {
            item: PipelineIn::default(),
        }
    }
}

pub struct PipelineInBuilder {
    item: PipelineIn,
}

impl PipelineInBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.item.name = name.to_string();
        self
    }

    pub fn config(mut self, config: &str) -> Self {
        self.item.config = Some(config.to_string());
        self
    }

    pub fn indexing_yaml(mut self, yaml: &str) -> Self {
        self.item.indexing_yaml = Some(yaml.to_string());
        self
    }

    pub fn query_yaml(mut self, yaml: &str) -> Self {
        self.item.query_yaml = Some(yaml.to_string());
        self
    }

    pub fn deepset_cloud_version(mut self, version: DeepsetCloudVersion) -> Self {
        self.item.deepset_cloud_version = version;
        self
    }

    pub fn build(self) -> PipelineIn {
        self.item
    }
}

#[derive(Debug, Deserialize, Serialize, Getters)]
pub struct PipelineOut {
    name: String,
}
