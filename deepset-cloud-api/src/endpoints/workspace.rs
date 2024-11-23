use apisdk::{send, ApiResult};
use tracing::instrument;

use crate::types::Workspace;
use crate::DeepsetCloudApi;

impl DeepsetCloudApi {
    #[instrument]
    pub async fn get_workspace(&self, name: &str) -> ApiResult<Workspace> {
        let req = self.get(format!("/api/v1/workspaces/{}", name)).await?;
        send!(req, Json).await
    }

    #[instrument]
    pub async fn list_workspaces(&self) -> ApiResult<Vec<Workspace>> {
        let req = self.get("/api/v1/workspaces").await?;
        send!(req, Json).await
    }
}
