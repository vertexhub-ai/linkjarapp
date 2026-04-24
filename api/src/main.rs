use axum::{routing::{get, post}, Router};

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/links", post(routes::create_link))
        .route("/:code", get(routes::redirect))
        .route("/links/:code/stats", get(routes::stats));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on :3000");
    axum::serve(listener, app).await.unwrap();
}
