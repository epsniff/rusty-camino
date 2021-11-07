extern crate rusty_camino;
use hyper::Server;
use rusty_camino::{routes, startup, ResultExt};
use routerify::RouterService;
use std::net::SocketAddr;
use rusty_camino::info_retrieval::canister;
use std::fs;
use rusty_camino::info_retrieval::types::IndexSettings;



#[tokio::main]
async fn main() {

    startup::up().await.context("Failed to startup the server").unwrap();

    log::info!("starting index server");

    let prefix_path = rusty_camino::config::server_canister_path();
    log::info!("Loading existing indexes from canister path {}", prefix_path.clone().to_str().unwrap());

    if !prefix_path.as_path().exists() {
        log::warn!("the canister path doesn't exist, creating it at {}", prefix_path.clone().to_str().unwrap());
        fs::create_dir(prefix_path.as_path());
    }

    let paths = fs::read_dir(prefix_path.clone()).unwrap();
    let can = canister();

    for path in paths {
        let pb = path.unwrap().path();
        let name =pb.strip_prefix(&prefix_path).unwrap();
        can.open_index(IndexSettings{
            index_name: String::from(name.to_str().unwrap()),
            writer_memory: 3000000,
            merge_policy: String::from("merge_log"),
         }).unwrap();
    }


    let addr = SocketAddr::new( 
        rusty_camino::config::server_bind_ip(),
        rusty_camino::config::server_bind_port(),
    );

    let server = Server::bind(&addr)
        .http1_keepalive(true)
        .http1_half_close(true)
        .http1_only(false)
        .http2_only(false)
        .http1_writev(true)
        .tcp_sleep_on_accept_errors(true)
        .serve(RouterService::new(routes::indexer_router()).unwrap());

    log::info!("Indexer is serving on: {}", server.local_addr());
    if let Err(e) = server.await {
        log::error!("Server Error: {}", e);
    }
}
