use std::convert::Infallible;

use crate::utils::helpers::{handle_404, handle_ping};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};

use crate::routers::books::books_router;

pub async fn base_router(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = req.uri().path();
    let base_path = path.split('/').nth(1).unwrap();

    match base_path {
        "" => handle_ping().await,
        "books" => books_router(req).await,
        _ => handle_404().await,
    }
}
