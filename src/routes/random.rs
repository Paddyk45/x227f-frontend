use crate::DATABASE;
use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};
use axum_limit::LimitPerSecond;

#[derive(Deserialize)]
pub struct QueryParams {
    amount: usize,
}

#[derive(Serialize)]
pub struct Hashes {
    hashes: Vec<String>,
}

pub async fn random_api_route(_: LimitPerSecond<2, http::Uri>, q: Query<QueryParams>) -> Json<Hashes> {
    let amount = q.amount.clamp(1, 1000);

    let db = {
        let db = DATABASE.lock().await;
        db.try_clone().unwrap()
    };

    let mut stmt = db
        .prepare(&format!(
            "SELECT sha256 FROM (SELECT DISTINCT sha256 FROM pages) USING SAMPLE {amount}"
        ))
        .unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut hashes: Vec<String> = vec![];
    while let Ok(Some(row)) = rows.next() {
        hashes.push(row.get(0).unwrap());
    }

    Json(Hashes { hashes })
}
