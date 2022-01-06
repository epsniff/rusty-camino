use crate::utils;
use hyper::{Body, Request, Response, StatusCode};
use routerify::prelude::*;

// POST /api/v1/:indexname/_batch
pub async fn index_batch_load(req: Request<Body>) -> crate::Result<Response<Body>> {
    let _index_name = req.param("indexname").unwrap();
    let res = utils::http::body_with_code(StatusCode::OK, {});
    Ok(res)
}

// GET /api/v1/:indexname/_stats
pub async fn index_stats(req: Request<Body>) -> crate::Result<Response<Body>> {
    let _index_name = req.param("indexname").unwrap();
    let res = utils::http::body_with_code(StatusCode::OK, {});
    Ok(res)
}
