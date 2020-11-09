use crate::prelude::*;
use hyper::{header::HeaderName, Body, Request};
use std::env;
use std::net::IpAddr;
use uuid::Uuid;

pub fn env(name: &str) -> Option<String> {
    env::var(name).ok()
}

pub fn env_crit(name: &str) -> String {
    env(name)
        .filter(|x| !x.trim().is_empty())
        .expect(format!(r#"The critical env variable: "{}" not given"#, name).as_str())
}

pub fn gen_uuid() -> String {
    Uuid::new_v4()
        .to_hyphenated()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_owned()
}

pub fn or<T: Sized>(cond: bool, truth_val: T, false_val: T) -> T {
    if cond {
        truth_val
    } else {
        false_val
    }
}

pub fn extract_client_ip_from_req(req: &Request<Body>) -> IpAddr {
    req.headers()
        .get(HeaderName::from_static("x-forwarded-for"))
        .and_then(|v| v.to_str().ok())
        .and_then(|ips| ips.split(",").nth(0))
        .map(|ip| ip.trim())
        .and_then(|ip| ip.parse::<IpAddr>().ok())
        .unwrap_or(req.remote_addr().ip())
}
