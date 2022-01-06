use hyper::{Response, StatusCode};
use hyper::Body;
use serde::Serialize;


pub fn error_with_code(code: StatusCode, err: crate::Error) -> Response<Body>
{
    body_with_code(code, err)
}

pub fn body_with_code<T>(code: StatusCode, body: T) -> Response<Body>
where
    T: Serialize,
{
    let json = serde_json::to_vec::<T>(&body).unwrap();

    Response::builder()
        .status(code)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))
        .unwrap()
}

pub fn empty_with_code(code: StatusCode) -> Response<Body> {
    Response::builder().status(code).body(Body::empty()).unwrap()
}
