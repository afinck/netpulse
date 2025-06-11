// filepath: /netpulse/netpulse/src/main.rs
use std::net::SocketAddr;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

mod db;
mod measurements;
mod pdf_export;
mod utils;
mod web;

use web::handlers::{AppState, create_routes};

#[tokio::main]
async fn main() {
    // Create a connection manager for SQLite
    let manager = SqliteConnectionManager::file("netpulse.db");
    let pool = Pool::new(manager).expect("Failed to create connection pool");
    let state = Arc::new(AppState { db: pool });

    // Router with state
    let app = create_routes(state);

    // Address festlegen
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    // Server starten
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}