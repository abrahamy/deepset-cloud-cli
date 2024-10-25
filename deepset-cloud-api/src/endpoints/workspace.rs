use apisdk::{send, ApiResult};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::types::Workspace;
use crate::DeepsetCloudApi;

#[cfg(feature = "dev-api")]
use crate::DeepsetCloudDevApi;

#[derive(Debug, Serialize, Deserialize)]
struct WorkspaceList {
    root: Vec<Workspace>,
}

impl DeepsetCloudApi {
    #[instrument]
    pub async fn get_workspace(&self, name: String) -> ApiResult<Workspace> {
        let req = self.get(format!("/api/v1/workspaces/{}", name)).await?;
        send!(req).await
    }

    #[instrument]
    pub async fn list_workspaces(&self) -> ApiResult<Vec<Workspace>> {
        let workspace_list = self.list_workspaces_().await?;
        Ok(workspace_list.root)
    }

    #[instrument]
    async fn list_workspaces_(&self) -> ApiResult<WorkspaceList> {
        let req = self.get("/api/v1/workspaces").await?;
        send!(req).await
    }
}

#[cfg(feature = "dev-api")]
impl DeepsetCloudDevApi {
    #[instrument]
    pub async fn get_workspace(&self, name: String) -> ApiResult<Workspace> {
        let req = self.get(format!("/api/v1/workspaces/{}", name)).await?;
        send!(req).await
    }

    #[instrument]
    pub async fn list_workspaces(&self) -> ApiResult<Vec<Workspace>> {
        let workspace_list = self.list_workspaces_().await?;
        Ok(workspace_list.root)
    }

    #[instrument]
    async fn list_workspaces_(&self) -> ApiResult<WorkspaceList> {
        let req = self.get("/api/v1/workspaces").await?;
        send!(req).await
    }
}
