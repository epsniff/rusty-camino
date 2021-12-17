use crate::routes::indexer_server::v1::index::controllers;
use hyper::{Body, Request, Response};
use hyper::body;
use routerify::prelude::*;
use std::str;

// POST /api/v1/:indexname/_create
pub async fn index_create(req: Request<Body>) -> crate::Result<Response<Body>> {
    let index_name = req.param("indexname").unwrap();
    // TODO (ES) use pattern matching on controller response 
    resp_200!(controllers::index_create(index_name).await?)
}

// POST /api/v1/:indexname/_upsert
/*
echo '{
...
}' | 

http POST localhost:8001/api/v1/foo/_upsert

*/
pub async fn index_single(req: Request<Body>) -> crate::Result<Response<Body>> {
    let index_name = req.param("indexname").unwrap();
    let res = body::to_bytes(req.into_body()).await;
    let bytes;
    match res {
        Ok(_bytes) => bytes = _bytes,
        Err(e) => resp_400!("Invalid body"),
    }
    let doc = match str::from_utf8(&bytes) { 
        Ok(doc) => doc,
        Err(e) => resp_400!("Invalid UTF-8 sequence"),
    };

    let res = controllers::index_single(index_name, doc).await;
    match res {
        Ok(result) => resp_200!(result),
        Err(e) => resp_400!("Invalid body" ),
    }
}

// POST /api/v1/:indexname/_batch
pub async fn index_batch_load(req: Request<Body>) -> crate::Result<Response<Body>> {
    let index_name = req.param("indexname").unwrap();
    resp_200!(controllers::index_batch_load(index_name).await?)
}

// POST /api/v1/:indexname/_search
pub async fn index_search(req: Request<Body>) -> crate::Result<Response<Body>> {
    let index_name = req.param("indexname").unwrap();
    resp_200!(controllers::index_search(index_name).await?)
}

// GET /api/v1/:indexname/_stats
pub async fn index_stats(req: Request<Body>) -> crate::Result<Response<Body>> {
    let index_name = req.param("indexname").unwrap();
    resp_200!(controllers::index_stats(index_name).await?)
}
