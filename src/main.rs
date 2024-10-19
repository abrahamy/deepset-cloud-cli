use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

use deepset_cloud_api::types::sdk::AccessTokenAuth;
use deepset_cloud_api::{DeepsetCloudApi, DeepsetCloudSettings};

fn configure_logging() {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .json()
            .finish(),
    )
    .expect("Setting default subscriber failed");
}

#[tokio::main]
async fn main() {
    configure_logging();
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
            error!("Failed to retrieve workspace, reason {}", api_error);
        }
    };
}
