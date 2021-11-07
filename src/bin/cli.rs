extern crate rusty_camino;
use rusty_camino::{startup, ResultExt};

#[tokio::main]
async fn main() {
    startup::up().await.context("Failed to startup the server").unwrap();
    log::info!("rusty-camino CLI tools: help == 'if you actually wrote these tools, I'd be a lot more help!!!'");
}