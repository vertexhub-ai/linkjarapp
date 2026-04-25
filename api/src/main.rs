use axum::{routing::{get, post}, Router};
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| format!("http://{}", listen_addr));

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = Arc::new(handlers::AppState { pool, base_url });

    let app = Router::new()
        .route("/links", post(handlers::create_link))
        .route("/links/:code/stats", get(handlers::stats))
        .route("/:code", get(handlers::redirect))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&listen_addr).await.unwrap();
    println!("Listening on {listen_addr}");
    axum::serve(listener, app).await.unwrap();
}
