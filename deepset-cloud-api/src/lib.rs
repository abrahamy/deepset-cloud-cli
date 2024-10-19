use apisdk::http_api;

mod endpoints;
mod settings;
pub mod types;

pub use self::settings::DeepsetCloudSettings;

#[http_api("https://api.cloud.deepset.ai/")]
pub struct DeepsetCloudApi;
