use crate::http_client::client;
use crate::prelude::*;
use hyper::header;
use serde_json::Value;
use std::{error::Error as StdError, fmt};
use url::Url;

pub async fn index_create(idx: &str) -> crate::Result<String> {
    Ok(format!(
        "index: {}, index_create",
        idx,
    ))
}

pub async fn index_single(idx: &str) -> crate::Result<String> {
    Ok(format!(
        "index: {}, index_single",
        idx,
    ))
}

pub async fn index_search(idx: &str) -> crate::Result<String> {
    Ok(format!(
        "index: {}, index_search",
        idx,
    ))
}

pub async fn index_batch_load(idx: &str) -> crate::Result<String> {
    Ok(format!(
        "index: {}, index_batch_load",
        idx,
    ))
}

pub async fn index_stats(_idx: &str) -> crate::Result<String> {
    // return Err(crate::Error::new(format!(
    //     "Not Implemented: index_stats, index: {} ",
    //     idx,
    // )));
    return Err(crate::ErrorExt::wrap(NotImplementedError));
    // Ok(format!(
    //     "index: {}, index_stats",
    //     idx
    // ))
}

#[derive(Debug)]
struct NotImplementedError;
impl StdError for NotImplementedError {}
impl fmt::Display for NotImplementedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NOT IMPLEMENT")
    }
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
