use std::convert::Infallible;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};

pub async fn handle_ping() -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Full::new(Bytes::from("Server Live.")))
        .unwrap())
}

pub async fn handle_404() -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(Bytes::from("Route Not Found.")))
        .unwrap())
}
