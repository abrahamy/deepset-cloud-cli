use apisdk::{send, ApiResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DeepsetCloudApi;

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    name: String,
    workspace_id: Uuid,
    languages: Vec<String>,
    default_idle_timeout_in_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorkspaceList {
    root: Vec<Workspace>,
}

impl DeepsetCloudApi {
    pub async fn get_workspace(&self, name: String) -> ApiResult<Workspace> {
        let req = self.get(format!("/api/v1/workspaces/{}", name)).await?;
        send!(req).await
    }

    pub async fn list_workspaces(&self) -> ApiResult<Vec<Workspace>> {
        let workspace_list = self.list_workspaces_().await?;
        Ok(workspace_list.root)
    }

    async fn list_workspaces_(&self) -> ApiResult<WorkspaceList> {
        let req = self.get("/api/v1/workspaces").await?;
        send!(req).await
    }
}
