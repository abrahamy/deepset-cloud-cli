use deepset_cloud_api::DeepsetCloudApi;

#[tokio::main]
async fn main() {
    let api = DeepsetCloudApi::default();
    let result = api.get_workspace("default".into()).await;
    match result {
        Ok(workspace) => {
            dbg!(workspace);
            ()
        }
        Err(api_error) => {
            dbg!(api_error);
            ()
        }
    }
    println!("Hello, world!");
}
