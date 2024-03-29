use crate::routes::indexer_server::v1::index::controllers;
use crate::utils;
use hyper::body;
use hyper::{Body, Request, Response, StatusCode};
use routerify::prelude::*;
use std::str;

// POST /api/v1/:indexname/_create
pub async fn index_create(req: Request<Body>) -> crate::Result<Response<Body>> {
    let index_name = req.param("indexname").unwrap();
    // TODO (ES) use pattern matching on controller response
    let con_res = controllers::index_create(index_name).await;
    let res = utils::http::body_with_code(StatusCode::OK, con_res);
    Ok(res)
}

// POST /api/v1/:indexname/_upsert
/*
echo '{
...
}' |

http POST localhost:8001/api/v1/foo/_upsert

*/
pub async fn index_single(req: Request<Body>) -> crate::Result<Response<Body>> {

    let (parts, body) = req.into_parts();
    let path = parse_path(parts.uri.path());
    let index_name = match &path[..] {
        [_, index_name, ..] => index_name,
        _ => {
            let cerr = crate::Error::new(format!("Failed to parse index name"));
            let res = utils::http::error_with_code(StatusCode::NOT_ACCEPTABLE, cerr);
            return Ok(res);
        }, 
    };

    // let index_name = req.clone().param("indexname").unwrap();
    let res = body::to_bytes(body).await;
    let bytes;
    match res {
        Ok(_bytes) => bytes = _bytes,
        Err(e) => {
            let cerr = crate::Error::new(format!("Invalid body: error:{}", e));
            let res = utils::http::error_with_code(StatusCode::NOT_ACCEPTABLE, cerr);
            return Ok(res);
        },
    }
    let doc = match str::from_utf8(&bytes) {
        Ok(doc) => doc,
        Err(e) => {
            let cerr = crate::Error::new(format!("Invalid UTF-8 sequence: error:{}", e));
            let res = utils::http::error_with_code(StatusCode::NOT_ACCEPTABLE, cerr);
            return Ok(res);
        },
    };

    let res = controllers::index_single(index_name, doc).await;
    match res {
        Ok(result) => {
            let res = utils::http::body_with_code(StatusCode::OK, result);
            return Ok(res);
        },
        Err(e) => {
            let cerr = crate::Error::new(format!("Invalid index request: error:{}", e));
            let res = utils::http::error_with_code(StatusCode::NOT_ACCEPTABLE, cerr);
            return Ok(res);
        },
    }
}

// POST /api/v1/:indexname/_batch
pub async fn index_batch_load(req: Request<Body>) -> crate::Result<Response<Body>> {
    let index_name = req.param("indexname").unwrap();
    let res = controllers::index_batch_load(index_name).await;
    match res {
        Ok(result) => {
            let res = utils::http::body_with_code(StatusCode::OK, result);
            return Ok(res);
        },
        Err(e) => {
            let cerr = crate::Error::new(format!("Invalid batch request: error:{}", e));
            let res = utils::http::error_with_code(StatusCode::NOT_ACCEPTABLE, cerr);
            return Ok(res);
        },
    }
}

fn parse_path(path: &str) -> Vec<&str> {
    path.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect()
}
// POST /api/v1/:indexname/_search
pub async fn index_search(req: Request<Body>) -> crate::Result<Response<Body>> {
    let (parts, body) = req.into_parts();
    let path = parse_path(parts.uri.path());
    let index_name = match &path[..] {
        [_, index_name, ..] => index_name,
        _ => {
            let cerr = crate::Error::new(format!("Failed to parse index name"));
            let res = utils::http::error_with_code(StatusCode::NOT_ACCEPTABLE, cerr);
            return Ok(res);
        }, 
    };
    
    let res = hyper::body::to_bytes(body).await;
    let bytes = match res {
        Ok(_bytes) => _bytes,
        Err(e) => {
            let cerr = crate::Error::new(format!("Invalid body: error:{}", e));
            let res = utils::http::error_with_code(StatusCode::NOT_ACCEPTABLE, cerr);
            return Ok(res);
        },
    };
    let query = match std::str::from_utf8(&bytes) { 
        Ok(_query) => _query,
        Err(e) => {
            let cerr = crate::Error::new(format!("Invalid UTF-8 sequence: error:{}", e));
            let res = utils::http::error_with_code(StatusCode::NOT_ACCEPTABLE, cerr);
            return Ok(res);
        },
    };

    let res = controllers::index_search(index_name, query).await;
    match res {
        Ok(result) => {
            let res = utils::http::body_with_code(StatusCode::OK, result);
            return Ok(res);
        },
        Err(e) => {
            let cerr = crate::Error::new(format!("Invalid search request: error:{}", e));
            let res = utils::http::error_with_code(StatusCode::NOT_ACCEPTABLE, cerr);
            return Ok(res);
        },
    }
}

// GET /api/v1/:indexname/_stats
pub async fn index_stats(req: Request<Body>) -> crate::Result<Response<Body>> {
    let index_name = req.param("indexname").unwrap();
    let con_res = controllers::index_stats(index_name).await;
    let res = utils::http::body_with_code(StatusCode::OK, con_res);
    Ok(res)
}
