use apisdk::http_api;

mod endpoints;
mod settings;
pub mod types;

pub use self::settings::DeepsetCloudSettings;

#[derive(Debug)]
#[http_api("https://api.cloud.deepset.ai/")]
pub struct DeepsetCloudApi;

#[cfg(feature = "dev-api")]
#[derive(Debug)]
#[http_api("https://api.dev.cloud.dpst.dev/")]
pub struct DeepsetCloudDevApi;
