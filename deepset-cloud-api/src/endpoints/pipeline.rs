use apisdk::{send, ApiResult};

use crate::types::Pipeline;
use crate::DeepsetCloudApi;

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
