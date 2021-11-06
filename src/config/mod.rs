

use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use crate::{constants, routes, startup, utils, ResultExt};


pub fn server_bind_ip() -> IpAddr {
    let bind_ip: IpAddr = utils::env(constants::env::HOST)
            .and_then(|host| host.parse::<IpAddr>().ok())
            .unwrap_or(constants::SERVER_DEFAULT_IP);
    return bind_ip
}

pub fn server_bind_port() -> u16 {
    let bind_port: u16 = utils::env(constants::env::PORT)
            .and_then(|port| port.parse::<u16>().ok())
            .unwrap_or(constants::SERVER_DEFAULT_PORT);
    return bind_port
}

pub fn server_canister_path() -> PathBuf {
    let path: PathBuf = utils::env(constants::env::CANISTER_PATH)
            .and_then(|p| Some(PathBuf::from(p)))
            .unwrap_or(PathBuf::from(constants::SERVER_DEFAULT_CANISTER_PATH));
    return path
}



