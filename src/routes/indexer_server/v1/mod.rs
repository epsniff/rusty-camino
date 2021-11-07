use hyper::Body;
use routerify::Router;

mod helpers;
mod index;

pub fn router() -> Router<Body, crate::Error> {
    Router::builder()
        // POST /v1/:indexname/_create
        .post("/:indexname/_create", index::handlers::index_create)
        // POST /v1/:indexname/_upsert
        .post("/:indexname/_upsert", index::handlers::index_single)
        // POST /v1/:indexname/_batch
        .post("/:indexname/_batch", index::handlers::index_batch_load)
        // POST /v1/:indexname/_search
        .post("/:indexname/_search", index::handlers::index_search)
        // GET /v1/:indexname/_stats
        .get("/:indexname/_stats", index::handlers::index_stats)
        .build()
        .unwrap()
}
