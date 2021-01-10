use crate::prelude::*;
use hyper::{body, Body, Request};
use serde::de::DeserializeOwned;

#[allow(dead_code)]
pub async fn parse_req_body_as_json<T: DeserializeOwned>(context: &str, req: Request<Body>) -> crate::Result<T> {
    let data = body::aggregate(req.into_body())
        .await
        .context(format!("{}: Failed to aggregate request body", context))?;

    serde_json::from_reader(data.reader()).context(format!("{}: Failed to decode request body as JSON", context))
}
