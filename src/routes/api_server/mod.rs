use hyper::Body;
use routerify::Router;

mod v1;

pub fn router() -> Router<Body, crate::Error> {
    Router::builder().scope("/v1", v1::router()).build().unwrap()
}
