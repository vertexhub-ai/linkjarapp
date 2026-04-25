use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use rand::Rng;
use sqlx::PgPool;
use std::sync::Arc;
use url::Url;

use crate::models::{CreateLinkRequest, CreateLinkResponse, ErrorResponse, Link, StatsResponse};

pub struct AppState {
    pub pool: PgPool,
    pub base_url: String,
}

const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn generate_code() -> String {
    let mut rng = rand::thread_rng();
    (0..6).map(|_| BASE62[rng.gen_range(0..62)] as char).collect()
}

fn is_valid_url(s: &str) -> bool {
    Url::parse(s)
        .map(|u| u.scheme() == "http" || u.scheme() == "https")
        .unwrap_or(false)
}

pub async fn create_link(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateLinkRequest>,
) -> impl IntoResponse {
    if !is_valid_url(&body.url) {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse { error: "Invalid URL: must be http or https".into() }),
        )
            .into_response();
    }

    let code = body.custom_code.unwrap_or_else(generate_code);

    let result = sqlx::query_as::<_, Link>(
        "INSERT INTO links (code, url) VALUES ($1, $2) RETURNING *",
    )
    .bind(&code)
    .bind(&body.url)
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok(link) => {
            let short_url = format!("{}/{}", state.base_url.trim_end_matches('/'), link.code);
            (
                StatusCode::CREATED,
                Json(CreateLinkResponse { code: link.code, short_url }),
            )
                .into_response()
        }
        Err(sqlx::Error::Database(e)) if e.code().as_deref() == Some("23505") => (
            StatusCode::CONFLICT,
            Json(ErrorResponse { error: "Code already in use".into() }),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: "Database error".into() }),
        )
            .into_response(),
    }
}

pub async fn redirect(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query_scalar::<_, String>(
        "UPDATE links SET hit_count = hit_count + 1 WHERE code = $1 RETURNING url",
    )
    .bind(&code)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(url)) => (
            StatusCode::MOVED_PERMANENTLY,
            [(header::LOCATION, url)],
        )
            .into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Link not found".into() }),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: "Database error".into() }),
        )
            .into_response(),
    }
}

pub async fn stats(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Link>("SELECT * FROM links WHERE code = $1")
        .bind(&code)
        .fetch_optional(&state.pool)
        .await;

    match result {
        Ok(Some(link)) => (
            StatusCode::OK,
            Json(StatsResponse {
                code: link.code,
                url: link.url,
                hit_count: link.hit_count,
                created_at: link.created_at,
            }),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Link not found".into() }),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: "Database error".into() }),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_code_is_six_chars() {
        assert_eq!(generate_code().len(), 6);
    }

    #[test]
    fn generate_code_only_base62_chars() {
        for _ in 0..200 {
            let code = generate_code();
            assert!(
                code.chars().all(|c| c.is_ascii_alphanumeric()),
                "non-base62 char in code: {code}"
            );
        }
    }

    #[test]
    fn valid_url_accepts_https() {
        assert!(is_valid_url("https://example.com/path?q=1"));
    }

    #[test]
    fn valid_url_accepts_http() {
        assert!(is_valid_url("http://example.com"));
    }

    #[test]
    fn valid_url_rejects_bare_string() {
        assert!(!is_valid_url("not-a-url"));
    }

    #[test]
    fn valid_url_rejects_ftp_scheme() {
        assert!(!is_valid_url("ftp://example.com"));
    }

    #[test]
    fn valid_url_rejects_empty() {
        assert!(!is_valid_url(""));
    }
}
