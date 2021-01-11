extern crate rusty_camino;

use rusty_camino::ResultExt;

#[tokio::test]
async fn test() {
    rusty_camino::startup::up().await.context("Failed to startup the server").unwrap();

    assert_eq!(1, 1);
}
