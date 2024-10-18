use apisdk::http_api;

mod endpoints;

#[http_api("https://api.cloud.deepset.ai/")]
pub struct DeepsetCloudApi;
