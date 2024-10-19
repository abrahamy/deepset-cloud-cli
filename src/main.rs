use deepset_cloud_api::types::sdk::AccessTokenAuth;
use deepset_cloud_api::{DeepsetCloudApi, DeepsetCloudSettings};

#[tokio::main]
async fn main() {
    let settings = DeepsetCloudSettings::init()
        .expect("Could not determine settings from environment variables!");

    let api = DeepsetCloudApi::builder()
        .with_authenticator(AccessTokenAuth::new(settings.api_key))
        .build();

    let result = api.get_workspace(settings.workspace_name).await;

    match result {
        Ok(workspace) => {
            dbg!(workspace);
        }
        Err(api_error) => {
            dbg!(api_error);
        }
    };
}
