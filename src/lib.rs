pub use self::error::{Error, ErrorExt, ResultExt};
#[cfg(test)]
pub use self::test_env::setup_test_environment;

#[macro_use]
pub mod logger;
#[macro_use]
pub mod macros;

pub mod constants;
mod error;
pub mod ext;
pub mod http_client;
pub mod prelude;
pub mod routes;
pub mod startup;
#[cfg(test)]
mod test_env;
pub mod types;
pub mod utils;

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_foo() {
        setup_test_env!();
        assert_eq!(1, 1);
    }
}
