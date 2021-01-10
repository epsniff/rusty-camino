use hyper::Body;
use routerify::Router;

mod helpers;
mod index;

pub fn router() -> Router<Body, crate::Error> {
    Router::builder()
        // POST /api/v1/:indexname/_batch
        .post("/:indexname/_batch", index::handlers::index_batch_load)
        // GET /api/v1/:indexname/_stats
        .get("/:indexname/_stats", index::handlers::index_stats)
        .build()
        .unwrap()
}
