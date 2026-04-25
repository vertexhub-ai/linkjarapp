use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Link {
    pub id: Uuid,
    pub code: String,
    pub url: String,
    pub hit_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLinkRequest {
    pub url: String,
    pub custom_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateLinkResponse {
    pub code: String,
    pub short_url: String,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub code: String,
    pub url: String,
    pub hit_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
