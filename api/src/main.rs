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

mod db;
mod event_handler;
mod models;

pub use event_handler::Event;

use crate::db::DB;
use axum::{routing::get, Router};
use surrealdb::engine::remote::ws::Wss;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the database
    DB.connect::<Wss>("localhost").await?;
    // Select a namespace + database
    DB.use_ns("shrimp").use_db("stats").await?;

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
