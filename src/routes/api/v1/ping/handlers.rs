use crate::routes::api::v1::ping::controllers;
use hyper::{Body, Request, Response};

pub async fn ping_get(_: Request<Body>) -> crate::Result<Response<Body>> {
    resp_200!(controllers::gen_ping_message().await?)
}

pub async fn pong_get(_: Request<Body>) -> crate::Result<Response<Body>> {
    resp_200!(controllers::gen_pong_message().await?)
}
