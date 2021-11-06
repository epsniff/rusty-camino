

use std::net::{IpAddr, SocketAddr};

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




