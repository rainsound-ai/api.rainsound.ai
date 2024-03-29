use routes::*;
use axum::Router;
use axum::routing::{get, post};
use tower_http::services::ServeDir;

mod routes;
mod open_ai;
mod image;
mod render;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/generate-notion-cover-image", post(generate_notion_cover_image))
        .route("/test-generate-notion-cover-image", get(test_generate_notion_cover_image))
        .route("/healthz", get(health_check))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found);

    // Run our app with hyper, listening globally on the specified port.
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host_and_port = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(host_and_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() {}
