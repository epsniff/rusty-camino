use hyper::Body;
use routerify::Router;

mod helpers;
mod ping;

pub fn router() -> Router<Body, crate::Error> {
    Router::builder()
        .get("/ping", ping::handlers::ping_get)
        .get("/pong", ping::handlers::pong_get)
        .build()
        .unwrap()
}
