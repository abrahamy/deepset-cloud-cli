use apisdk::{send, send_json, ApiResult};
use tracing::instrument;

use crate::types::{Pipeline, PipelineIn, PipelineOut};
use crate::DeepsetCloudApi;

#[cfg(feature = "dev-api")]
use crate::DeepsetCloudDevApi;

impl DeepsetCloudApi {
    #[instrument]
    pub async fn get_pipeline(&self, name: &str, workspace_name: &str) -> ApiResult<Pipeline> {
        let req = self
            .get(format!(
                "/api/v1/workspaces/{}/pipelines/{}",
                workspace_name, name
            ))
            .await?;
        send!(req).await
    }

    #[instrument]
    pub async fn create_pipeline(
        &self,
        workspace_name: &str,
        payload: &PipelineIn,
    ) -> ApiResult<PipelineOut> {
        let req = self
            .post(format!("/api/v1/workspaces/{}/pipelines", workspace_name))
            .await?;
        send_json!(req, payload, Json).await
    }

    #[instrument]
    pub async fn update_pipeline_yaml(
        &self,
        workspace_name: &str,
        payload: &PipelineIn,
    ) -> ApiResult<PipelineOut> {
        let req = self
            .put(format!(
                "/api/v1/workspaces/{}/pipelines/${}/yaml",
                workspace_name,
                payload.name()
            ))
            .await?;
        send_json!(req, payload, Json).await
    }

    #[instrument]
    pub async fn validate_pipeline(
        &self,
        workspace_name: &str,
        payload: &PipelineIn,
    ) -> ApiResult<()> {
        let req = self
            .post(format!(
                "/api/v1/workspaces/{}/pipeline_validations",
                workspace_name
            ))
            .await?;
        send_json!(req, payload, ()).await
    }
}

#[cfg(feature = "dev-api")]
impl DeepsetCloudDevApi {
    #[instrument]
    pub async fn get_pipeline(&self, name: &str, workspace_name: &str) -> ApiResult<Pipeline> {
        let req = self
            .get(format!(
                "/api/v1/workspaces/{}/pipelines/{}",
                workspace_name, name
            ))
            .await?;
        send!(req).await
    }

    #[instrument]
    pub async fn create_pipeline(
        &self,
        workspace_name: &str,
        payload: &PipelineIn,
    ) -> ApiResult<PipelineOut> {
        let req = self
            .post(format!("/api/v1/workspaces/{}/pipelines", workspace_name))
            .await?;
        send_json!(req, payload, Json).await
    }

    #[instrument]
    pub async fn update_pipeline_yaml(
        &self,
        workspace_name: &str,
        payload: &PipelineIn,
    ) -> ApiResult<PipelineOut> {
        let req = self
            .put(format!(
                "/api/v1/workspaces/{}/pipelines/${}/yaml",
                workspace_name,
                payload.name()
            ))
            .await?;
        send_json!(req, payload, Json).await
    }

    #[instrument]
    pub async fn validate_pipeline(
        &self,
        workspace_name: &str,
        payload: &PipelineIn,
    ) -> ApiResult<()> {
        let req = self
            .post(format!(
                "/api/v1/workspaces/{}/pipeline_validations",
                workspace_name
            ))
            .await?;
        send_json!(req, payload, ()).await
    }
}
