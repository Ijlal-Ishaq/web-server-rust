use std::convert::Infallible;

use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::{Request, Response, StatusCode};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::mongo;

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    title: String,
    author: String,
}

pub async fn get_books() -> Result<Response<Full<Bytes>>, Infallible> {
    let mongo = mongo::initialize_mongo().await;

    let mut books_cursor = mongo
        .collection("store", "books")
        .find(None, None)
        .await
        .unwrap();

    let mut books = Vec::new();

    while books_cursor.advance().await.unwrap() {
        let book = books_cursor.deserialize_current().unwrap();
        books.push(book);
    }

    let books_json = serde_json::to_string(&books).expect("Failed to serialize to JSON");

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(books_json)))
        .unwrap())
}

pub async fn add_book(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let mongo = mongo::initialize_mongo().await;

    let body_bytes = req.collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
    let title = parsed_json["title"].as_str().unwrap_or_default();
    let author = parsed_json["author"].as_str().unwrap_or_default();

    let book = Book {
        title: title.to_string(),
        author: author.to_string(),
    };

    let book_collection: Collection<Book> = mongo.client.database("store").collection("books");
    let result = book_collection.insert_one(&book, None).await.unwrap();

    let result_json = serde_json::to_string(&result).expect("Failed to serialize to JSON");

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(result_json)))
        .unwrap())
}

pub async fn update_book(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let mongo = mongo::initialize_mongo().await;

    let body_bytes = req.collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
    let id = parsed_json["id"].as_str().unwrap_or_default();
    let title = parsed_json["title"].as_str().unwrap_or_default();
    let author = parsed_json["author"].as_str().unwrap_or_default();

    let book_collection: Collection<Book> = mongo.client.database("store").collection("books");

    let id = ObjectId::parse_str(id).unwrap();
    let filter_doc = doc! { "_id": id };
    let update_doc = doc! {
            "$set": doc! {
                "title": title.to_string(),
                "author": author.to_string(),
            }
    };

    let result = book_collection
        .update_one(filter_doc, update_doc, None)
        .await
        .unwrap();

    let result_json = serde_json::to_string(&result).expect("Failed to serialize to JSON");

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(result_json)))
        .unwrap())
}

pub async fn delete_book(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let mongo = mongo::initialize_mongo().await;

    let body_bytes = req.collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
    let id = parsed_json["id"].as_str().unwrap_or_default();

    let book_collection: Collection<Book> = mongo.client.database("store").collection("books");

    let id = ObjectId::parse_str(id).unwrap();
    let filter_doc = doc! { "_id": id };

    let result = book_collection.delete_one(filter_doc, None).await.unwrap();

    let result_json = serde_json::to_string(&result).expect("Failed to serialize to JSON");

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(result_json)))
        .unwrap())
}
