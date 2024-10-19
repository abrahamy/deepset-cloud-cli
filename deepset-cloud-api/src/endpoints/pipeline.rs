use apisdk::{send, ApiResult};
use tracing::instrument;

use crate::types::Pipeline;
use crate::DeepsetCloudApi;

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
}
