use crate::http_client::client;
use crate::prelude::*;
use crate::info_retrieval::canister;
use crate::info_retrieval::types::IndexSettings;
use hyper::header;
use serde_json::Value;
use std::{error::Error as StdError, fmt};
use url::Url;

static DEFAULT_SCHEMA: &str = r#"
[
    {
        "type": "text",
        "column_name": "a_foo",
        "stored": true,
        "indexed": true,
        "indexed_lang_stem": "en",
        "indexed_tokenized": true, 
        "indexed_tokenized_with_freqs_positions": true,
        "indexed_tokenized_with_freqs": true
    },
    {
        "type": "keyword",
        "column_name": "b_foo",
        "stored": true
    },
    {
        "type": "uint64",
        "column_name": "c_foo",
        "stored": true,
        "doc_values": true, 
        "indexed": true
    },
    {
        "type": "int64",
        "column_name": "d_foo",
        "stored": true,
        "doc_values": true, 
        "indexed": true
    },
    {
        "type": "float64",
        "column_name": "e_foo",
        "stored": true,
        "doc_values": true, 
        "indexed": true
    },
    {
        "type": "date",
        "column_name": "f_foo",
        "stored": true,
        "doc_values": true, 
        "indexed": true
    },
    {
        "type": "facet",
        "column_name": "g_foo"
    },
    {
        "type": "bytes",
        "column_name": "h_foo"
    }
]   "#;

pub async fn index_create(idx: &str) -> crate::Result<String> {
    let can = canister();
    can.add_index(DEFAULT_SCHEMA, IndexSettings{
        index_name: String::from(idx),
        writer_memory: 3000000,
        merge_policy: String::from("merge_log"),
     })
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
