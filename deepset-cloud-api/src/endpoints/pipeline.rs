use apisdk::{send, ApiResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DeepsetCloudApi;

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

#[derive(Debug, Serialize, Deserialize)]
pub enum DeepsetCloudVersion {
    V1,
    V2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineIndexingStatus {
    pending_file_count: u64,
    failed_file_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
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

impl DeepsetCloudApi {
    pub async fn get_pipeline(&self, name: &str, workspace_name: &str) -> ApiResult<Pipeline> {
        let req = self
            .get(format!(
                "/api/v1/workspaces/{}/pipelines/{}",
                workspace_name, name
            ))
            .await?;
        send!(req).await
    }
}
