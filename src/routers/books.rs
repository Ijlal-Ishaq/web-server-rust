use std::convert::Infallible;

use crate::utils::helpers::handle_404;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Method, Request, Response};

use crate::services::books::{add_book, delete_book, get_books, update_book};

pub async fn books_router(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = req.uri().path().trim_end_matches('/');
    let method = req.method();

    match (method, path) {
        (&Method::GET, "/books") => get_books().await,
        (&Method::POST, "/books/add") => add_book(req).await,
        (&Method::POST, "/books/update") => update_book(req).await,
        (&Method::POST, "/books/delete") => delete_book(req).await,
        _ => handle_404().await,
    }
}
