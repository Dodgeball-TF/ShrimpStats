#![deny(
    unsafe_code,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}