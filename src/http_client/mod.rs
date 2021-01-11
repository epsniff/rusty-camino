use crate::constants;
use lazy_static::lazy_static;
use reqwest::Client;

lazy_static! {
    static ref HTTP_CLIENT: Client = Client::builder()
        .user_agent(format!("{}/{}", constants::APP_NAME, constants::APP_VERSION))
        .build()
        .expect("Failed to create the global HTTP client instance");
}

pub fn client() -> &'static Client {
    &*HTTP_CLIENT
}
