use crate::DATABASE;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use duckdb::params_from_iter;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum SearchApiResponse {
    #[serde(rename = "results")]
    Results(Vec<String>),
    #[serde(rename = "error")]
    Error(String),
}

#[derive(Deserialize)]
pub struct QueryParams {
    query: String,
    search_in: u8,
}

pub async fn search_api_route(q: Query<QueryParams>) -> (StatusCode, Json<SearchApiResponse>) {
    let wildcards = q.query.chars().fold((0, 0), |mut p_u, c| {
        if c == '%' {
            p_u.0 += 1
        } else if c == '_' {
            p_u.1 += 1
        }
        p_u
    });

    if wildcards.0 > 3 || wildcards.1 > 10 {
        return (
            StatusCode::BAD_REQUEST,
            Json(SearchApiResponse::Error(
                "Too many wildcards. Max. 3 of % and 10 of _".to_string(),
            )),
        );
    }

    if q.search_in == 0 || q.search_in > 0b00001111 {
        return (
            StatusCode::BAD_REQUEST,
            Json(SearchApiResponse::Error(
                "Invalid bitflags. Did you check at least one search?".to_string(),
            )),
        );
    }

    let db = {
        let db = DATABASE.lock().await;
        db.try_clone().unwrap()
    };
    let search_filenames = q.search_in & (1 << 3) > 0;
    let search_names = q.search_in & (1 << 2) > 0;
    let search_links_to = q.search_in & (1 << 1) > 0;
    let search_backlinks = q.search_in & (1 << 0) > 0;

    let mut exprs = vec![];
    let mut n_exprs = 0;
    
    if search_filenames {
        exprs.push("filename LIKE ?");
        n_exprs += 1;
    }
    
    if search_names {
        exprs.push("alt LIKE ?");
        exprs.push("title LIKE ?");
        n_exprs += 2;
    }
    
    if search_links_to {
        exprs.push("link LIKE ?");
        n_exprs += 1;
    }
    
    if search_backlinks {
        exprs.push("page LIKE ?");
        n_exprs += 1;
    }

    let exprs = exprs.join(" OR ");
    let mut stmt = db
        .prepare(&format!(
            "SELECT DISTINCT sha256 FROM pages WHERE ({exprs}) LIMIT 1000"
        ))
        .unwrap();
    let hashes = stmt
        .query_map(params_from_iter(vec![&q.query; n_exprs]), |r| r.get::<_, String>(0))
        .unwrap()
        .map(|h| h.unwrap())
        .collect();

    (StatusCode::OK, Json(SearchApiResponse::Results(hashes)))
}
