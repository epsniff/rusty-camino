use crate::routes::indexer_server::v1::index::controllers;
use hyper::{Body, Request, Response};
use routerify::prelude::*;

// POST /api/v1/:indexname/_create
pub async fn index_create(req: Request<Body>) -> crate::Result<Response<Body>> {
    let index_name = req.param("indexname").unwrap();
    // TODO (ES) use pattern matching on controller response 
    resp_200!(controllers::index_create(index_name).await?)
}

// POST /api/v1/:indexname/_upsert
pub async fn index_single(req: Request<Body>) -> crate::Result<Response<Body>> {
    let index_name = req.param("indexname").unwrap();
    resp_200!(controllers::index_single(index_name).await?)
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
