extern crate rusty_camino;
use hyper::Server;
use rusty_camino::{constants, routes, startup, utils, ResultExt};
use routerify::RouterService;
use std::net::{IpAddr, SocketAddr};
use rusty_camino::info_retrieval::canister;
use std::fs;
use rusty_camino::info_retrieval::types::IndexSettings;

#[tokio::main]
async fn main() {
    startup::up().await.context("Failed to startup the server").unwrap();
let mut fuckme = constants::env::CANISTER_PATH.to_owned();
fuckme.push_str("/");
    let mut prefix_path = std::path::PathBuf::new();
    prefix_path.push(fuckme);
    let paths = fs::read_dir(constants::env::CANISTER_PATH).unwrap();
    let can = canister();
    for path in paths {
        let pb = path.unwrap().path();
        let name =pb.strip_prefix(&prefix_path).expect(&format!("{}", prefix_path.to_str().unwrap()));
        println!("{:?}", name.to_str());
    //can.add_index("", IndexSettings{
        //index_name: String::from(name.to_str().unwrap()),
        //writer_memory: 3000000,
        //merge_policy: String::from("merge_log"),
     //}).unwrap();
    }


    let addr = SocketAddr::new( 
        utils::env(constants::env::HOST)
            .and_then(|host| host.parse::<IpAddr>().ok())
            .unwrap_or(constants::SERVER_DEFAULT_IP),
        utils::env(constants::env::PORT)
            .and_then(|port| port.parse::<u16>().ok())
            .unwrap_or(constants::SERVER_DEFAULT_PORT),
    );

    let server = Server::bind(&addr)
        .http1_keepalive(true)
        .http1_half_close(true)
        .http1_only(false)
        .http2_only(false)
        .http1_writev(true)
        .tcp_sleep_on_accept_errors(true)
        .serve(RouterService::new(routes::indexer_router()).unwrap());

    rusty_camino::info!("Indexer is serving on: {}", server.local_addr());
    if let Err(e) = server.await {
        rusty_camino::error!("Server Error: {}", e);
    }
}