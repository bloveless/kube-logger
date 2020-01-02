use kube::{
    api::Api,
    client::APIClient,
    config,
};
use kube::api::{LogParams, RawApi};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=trace");

    let config = config::load_kube_config().await?;
//    let client = APIClient::new(config);

    let mut lp = LogParams::default();
    lp.container = Some("php".to_string());

    let lr = RawApi::v1Pod()
        .within("fritzandandre")
        .log("fritzandandre-php-0", &lp)?;

    let (parts, body) = lr.into_parts();
    let uri_str = format!("{}{}", config.base_path, parts.uri);
    // trace!("{} {}", parts.method, uri_str);
    // trace!("Request body: {:?}", String::from_utf8_lossy(&body));
    let stream = config.client.get(&uri_str);

    let stream = reqwest::Client::builder()
        .build()?
        .get(&uri_str)
        .send().await;
//    let mut stream = reqwest::get("http://httpbin.org/ip")
//        .await?
//        .bytes_stream();

    while let Some(item) = stream.next().await {
        println!("Chunk: {:?}", item?);
    }

    let req = stream.headers(parts.headers);

    while let Some(item) = req.next().await {
        println!("Chunk: {}", item);
    }


    //trace!("Request Headers: {:?}", req.headers());

    // client is a Reqwest client
    // let res = config.client.execute(req).await?;

    // res.bytes_stream().await {

    // }

    // res.next().await 

    // println!("Res: {:?}", res);
    // println!("Body: {}", res.text().await);

    // Manage pods

//    let pods = Api::v1Pod(client).within("fritzandandre");
//    let log = pods.log("fritzandandre-php-0", &lp).await?;

//    println!("Fna Logs: {}", log);

//    let fna = pods.get("fritzandandre-php-0").await?;

//    let req = self.api.log(name, lp)?;

    let mut stream = reqwest::get("http://httpbin.org/ip")
        .await?
        .bytes_stream();

    while let Some(item) = stream.next().await {
        println!("Chunk: {:?}", item?);
    }

    Ok(())
}
