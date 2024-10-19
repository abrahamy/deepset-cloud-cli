use apisdk::http_api;

mod endpoints;
pub mod types;

#[http_api("https://api.cloud.deepset.ai/")]
pub struct DeepsetCloudApi;
