use crate::constants;
use crate::types::AppInfo;
use crate::utils;
use hyper::{Body, Request, Response, StatusCode};
use routerify::{Middleware, Router};
use routerify_cors::enable_cors_all;

mod api_server;
mod indexer_server;

pub fn api_router() -> Router<Body, crate::Error> {
    Router::builder()
        .middleware(Middleware::pre(logger_middleware))
        .middleware(enable_cors_all())
        .get("/", home_get)
        .scope("/", api_server::router())
        .err_handler(error_handler)
        .build()
        .unwrap()
}

pub fn indexer_router() -> Router<Body, crate::Error> {
    Router::builder()
        .middleware(Middleware::pre(logger_middleware))
        .middleware(enable_cors_all())
        .get("/", home_get)
        .scope("/", indexer_server::router())
        .err_handler(error_handler)
        .build()
        .unwrap()
}

async fn logger_middleware(req: Request<Body>) -> crate::Result<Request<Body>> {
    log::info!(
        "{} {} {}",
        utils::extract_client_ip_from_req(&req),
        req.method(),
        req.uri()
    );
    Ok(req)
}

async fn home_get(_: Request<Body>) -> crate::Result<Response<Body>> {
    let res = utils::http::body_with_code(
        StatusCode::OK,
        AppInfo {
            name: constants::APP_NAME,
            version: constants::APP_VERSION,
            description: constants::APP_DESCRIPTION,
        },
    );
    return Ok(res);
}

async fn error_handler(err: routerify::Error) -> Response<Body> {
    log::error!("{}", err);
    let cerr = crate::Error::new(format!("Error: {}", err));
    utils::http::error_with_code(StatusCode::BAD_REQUEST, cerr)
}
