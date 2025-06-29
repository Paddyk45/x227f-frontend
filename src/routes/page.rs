use crate::DATABASE;
use axum::extract::Path;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub enum PageApiResponse {
    #[serde(rename = "info")]
    Info(PageInfo),
}

#[derive(Serialize)]
pub struct PageAndButton {
    page: String, 
    button: String,
    alt: Option<String>,
    title: Option<String>,
    filename: Option<String>,
}

#[derive(Serialize)]
pub struct MaybePageAndButton {
    page: Option<String>,
    button: String,
    alt: Option<String>,
    title: Option<String>,
    filename: Option<String>,
}

#[derive(Serialize)]
pub struct PageInfo {
    backlinks: Vec<PageAndButton>,
    buttons: Vec<MaybePageAndButton>,
}

pub async fn page_api_route(page: Path<String>) -> Json<PageApiResponse> {
    let page = page.0;
    let db = {
        let db = DATABASE.lock().await;
        db.try_clone().unwrap()
    };

    // Get pages that link to this site
    let mut stmt = db
        .prepare("SELECT page,sha256,alt,title,filename FROM pages WHERE link = ?")
        .unwrap();
    let backlinks = stmt
        .query_map([&page], |r| {
            Ok(PageAndButton {
                page: r.get::<_, String>(0).unwrap(),
                button: r.get::<_, String>(1).unwrap(),
                alt: r.get::<_, String>(2).ok(),
                title: r.get::<_, String>(3).ok(),
                filename: r.get::<_, String>(4).ok(),
            })
        })
        .unwrap();
    let backlinks = backlinks.map(Result::unwrap).collect::<Vec<PageAndButton>>();

    // Get pages that this site links to
    let mut stmt = db
        .prepare("SELECT link,sha256,alt,title,filename FROM pages WHERE page = ?")
        .unwrap();
    let buttons = stmt
        .query_map([&page], |r| {
            Ok(MaybePageAndButton {
                page: r.get::<_, String>(0).ok(),
                button: r.get::<_, String>(1).unwrap(),
                alt: r.get::<_, String>(2).ok(),
                title: r.get::<_, String>(3).ok(),
                filename: r.get::<_, String>(4).ok(),
            })
        })
        .unwrap();
    let buttons = buttons
        .map(Result::unwrap)
        .collect::<Vec<MaybePageAndButton>>();

    Json(PageApiResponse::Info(PageInfo { backlinks, buttons }))
}
