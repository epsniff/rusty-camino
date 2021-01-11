use std::net::IpAddr;
use std::net::Ipv4Addr;

pub mod env;

pub const SERVER_DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
pub const SERVER_DEFAULT_PORT: u16 = 3001;

pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
