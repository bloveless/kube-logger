use kube::{
    api::Api,
    client::APIClient,
    config,
};
use kube::api::{LogParams, RawApi};
use futures::{FutureExt, Stream, future::IntoStream, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=trace");

    let config = config::load_kube_config().await?;
    let client = APIClient::new(config);

    // Manage pods
    let pods = Api::v1Pod(client).within("fritzandandre");
    let pods2 = RawApi::v1Pod().within("fritzandandre");

    let mut lp = LogParams::default();
    lp.container = Some("php".to_string());
//    lp.follow = true;
    lp.tail_lines = Some(100);

    let log_string = pods.log("fritzandandre-php-0", &lp).await?;
    println!("FnA Log: {}", log_string);

    Ok(())
}
