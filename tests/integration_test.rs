extern crate rusty_camino;

#[tokio::test]
async fn test() {
    rusty_camino::setup_test_environment();

    assert_eq!(1, 1);
}
