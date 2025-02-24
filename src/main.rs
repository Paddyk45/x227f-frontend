mod routes;

use crate::routes::{
    button_api_route, page_api_route, random_api_route, search_api_route, stats_api_route,
};
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::{routing::get, Router};
use duckdb::Connection;
use once_cell::sync::Lazy;
use axum_limit::{Limit, LimitState};

use std::cell::OnceCell;
use std::collections::HashMap;
use std::fmt::format;
use std::sync::{Arc, OnceLock};

use tokio::fs::read;
use tokio::sync::Mutex;

pub static DATABASE: Lazy<Mutex<Connection>> =
    Lazy::new(|| Mutex::new(Connection::open("88x31.ddb").unwrap()));
pub static BUTTONS: OnceLock<HashMap<String, (String, Vec<u8>)>> = OnceLock::new();

#[tokio::main]
async fn main() {
    let mut images = HashMap::new();
    let mut paths = tokio::fs::read_dir("./buttons").await.unwrap();
    while let Ok(Some(path)) = paths.next_entry().await {
        let filename = path.file_name().to_string_lossy().to_string();
        let (sha256, ext) = filename.split_once(".").unwrap();
        let file = read(format!("./buttons/{filename}")).await.unwrap();
        images.insert(sha256.to_string(), (ext.to_string(), file));
    }
    BUTTONS.set(images).unwrap();
    // build our application with a route
    let app = Router::new()
        .route("/", get(index_route))
        .route("/style.css", get(style_route))
        .route("/button", get(button_route))
        .route("/random", get(random_route))
        .route("/page", get(page_route))
        .route("/search", get(search_route))
        .route("/api/page/:page", get(page_api_route))
        .route("/api/button/:hash", get(button_api_route))
        .route("/api/button/:hash/img", get(button_img_route))
        .route("/api/search", get(search_api_route))
        .route("/api/random", get(random_api_route))
        .route("/api/stats", get(stats_api_route))
        .with_state(LimitState::<http::Uri>::default())
        ;
    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8831").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index_route() -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html".parse().unwrap());
    (headers, include_str!("../static/index.html").to_string())
}

async fn style_route() -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/css".parse().unwrap());
    (headers, include_str!("../static/style.css").to_string())
}

async fn button_route() -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html".parse().unwrap());
    (headers, include_str!("../static/button.html").to_string())
}

async fn random_route() -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html".parse().unwrap());
    (headers, include_str!("../static/random.html").to_string())
}

async fn page_route() -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html".parse().unwrap());
    (headers, include_str!("../static/page.html").to_string())
}

async fn search_route() -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html".parse().unwrap());
    (headers, include_str!("../static/search.html").to_string())
}

async fn button_img_route(Path(hash): Path<String>) -> (StatusCode, HeaderMap, Vec<u8>) {
    let button = BUTTONS.get().unwrap().get(&hash);
    let button = match button {
        Some(button) => button.clone(),
        None => return (StatusCode::NOT_FOUND, HeaderMap::new(), vec![]),
    };

    let mut hm = HeaderMap::new();
    hm.insert(
        "content-type",
        format!("image/{}", button.0).parse().unwrap(),
    );
    (StatusCode::OK, hm, button.1.clone())
}
