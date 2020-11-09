use crate::http_client::client;
use crate::prelude::*;
use hyper::header;
use serde_json::Value;
use url::Url;

pub async fn gen_ping_message() -> crate::Result<String> {
    Ok("Hello Ping".to_string())
}

pub async fn gen_pong_message() -> crate::Result<String> {
    Ok("Hello Pong".to_string())
}

#[allow(dead_code)]
pub async fn make_a_http_req() -> crate::Result<Value> {
    let api_url = Url::parse("http://dummy.restapiexample.com/api/v1/employees").wrap()?;

    let resp = client()
        .get(api_url)
        .header(header::ACCEPT, "application/json")
        .send()
        .await
        .context("Failed to send a request to the API server")?
        .shake("Dummy API server")
        .await?;

    let data = resp
        .json::<Value>()
        .await
        .context("Failed to decode response body as JSON")?;

    Ok(data)
}
