use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateLinkRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct CreateLinkResponse {
    pub code: String,
    pub url: String,
    pub short_url: String,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub code: String,
    pub url: String,
    pub clicks: i64,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn create_link(
    Json(_body): Json<CreateLinkRequest>,
) -> impl IntoResponse {
    // TODO: persist to database and generate short code
    (
        StatusCode::CREATED,
        Json(CreateLinkResponse {
            code: "stub".into(),
            url: "https://example.com".into(),
            short_url: "http://localhost:3000/stub".into(),
        }),
    )
}

pub async fn redirect(Path(_code): Path<String>) -> impl IntoResponse {
    // TODO: look up code in database, increment visits, redirect
    Redirect::temporary("https://example.com")
}

pub async fn stats(Path(_code): Path<String>) -> impl IntoResponse {
    // TODO: look up code in database and return stats
    Json(StatsResponse {
        code: "stub".into(),
        url: "https://example.com".into(),
        clicks: 0,
        created_at: "2026-04-22T00:00:00Z".into(),
    })
}
