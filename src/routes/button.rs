use crate::DATABASE;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use duckdb::types::Value;
use serde::Serialize;

#[derive(Serialize)]
pub enum ButtonApiResponse {
    #[serde(rename = "info")]
    Button(ButtonInfo),
    #[serde(rename = "error")]
    Error(String),
}

#[derive(Serialize)]
pub struct ButtonInfo {
    pages: Vec<PageInfo>,
}

#[derive(Serialize)]
pub struct PageInfo {
    backlink: String,
    link: Option<String>,
    alt: Option<String>,
    title: Option<String>,
    filename: Option<String>,
}

pub async fn button_api_route(Path(hash): Path<String>) -> (StatusCode, Json<ButtonApiResponse>) {
    // make sure that the hash is a valid hash
    if hash.len() > 32 || hash.chars().any(|c| !c.is_ascii_hexdigit()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ButtonApiResponse::Error("Invalid hash".to_string())),
        );
    }
	
    let db = {
        let db = DATABASE.lock().await;
        db.try_clone().unwrap()
    };
    
    let mut stmt = db
        .prepare("SELECT page,link,alt,title,filename FROM pages WHERE sha256 = ?")
        .unwrap();

    let rows = stmt
        .query_map([hash], |r| {
            let Value::Text(backlink) = r.get(0).unwrap() else {
                unreachable!()
            };
            let link = if let Value::Text(t) = r.get(1).unwrap() {
                Some(t)
            } else {
                None
            };
            let alt = if let Value::Text(t) = r.get(2).unwrap() {
                Some(t)
            } else {
                None
            };
            let title = if let Value::Text(t) = r.get(3).unwrap() {
                Some(t)
            } else {
                None
            };
            let filename = if let Value::Text(t) = r.get(4).unwrap() {
                Some(t)
            } else {
                None
            };

            Ok(PageInfo {
                backlink,
                link,
                alt,
                title,
                filename,
            })
        })
        .unwrap();

    let pages = rows.map(Result::unwrap).collect();

    (
        StatusCode::OK,
        Json(ButtonApiResponse::Button(ButtonInfo { pages })),
    )
}
