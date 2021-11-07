extern crate rusty_camino;
use hyper::Server;
use rusty_camino::{routes, startup, ResultExt};
use routerify::RouterService;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    startup::up().await.context("Failed to startup the server").unwrap();

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
        .serve(RouterService::new(routes::api_router()).unwrap());

    log::info!("API server is serving on: {}", server.local_addr());
    if let Err(e) = server.await {
        log::error!("Server Error: {}", e);
    }
}
