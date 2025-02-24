use crate::DATABASE;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct StatsApiResponse {
    buttons_link: usize,
    buttons_no_link: usize,
    unique_buttons: usize,
}

pub async fn stats_api_route() -> Json<StatsApiResponse> {
    let db = {
        let db = DATABASE.lock().await;
        db.try_clone().unwrap()
    };

    let mut stmt = db.prepare("SELECT (SELECT count(*) FROM pages WHERE link IS NOT NULL),(SELECT count(*) FROM pages WHERE link IS NULL);").unwrap();
    let (buttons_link, buttons_no_link) = stmt
        .query_row([], |r| {
            Ok((r.get::<_, usize>(0).unwrap(), r.get::<_, usize>(1).unwrap()))
        })
        .unwrap();

    let mut stmt = db
        .prepare("SELECT count(*) FROM (SELECT DISTINCT sha256 FROM pages);")
        .unwrap();
    let unique_buttons = stmt.query_row([], |r| r.get::<_, usize>(0)).unwrap();
    
    Json(StatsApiResponse {
        buttons_link,
        buttons_no_link,
        unique_buttons,
    })
}
