use std::sync::Once;

static TEST_ENV_INIT: Once = Once::new();

pub fn setup_test_environment() {
    TEST_ENV_INIT.call_once(|| {
        dotenv::dotenv().ok();
    });
}
