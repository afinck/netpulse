// filepath: /netpulse/netpulse/src/main.rs
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use reqwest::Client;
use std::time::Instant;

mod db;
mod measurements;
mod pdf_export;
mod utils;
mod web;

use web::handlers::{AppState, create_routes};

async fn measure_bandwidth() -> Result<f64, reqwest::Error> {
    let url = "https://speed.hetzner.de/100MB.bin"; // Example test file
    let client = Client::new();
    let start = Instant::now();
    let resp = client.get(url).send().await?;
    let bytes = resp.bytes().await?.len();
    let elapsed = start.elapsed().as_secs_f64();
    let mbits = (bytes as f64 * 8.0) / 1_000_000.0;
    Ok(mbits / elapsed) // Mbit/s
}

#[tokio::main]
async fn main() {
    // Create a connection manager for SQLite
    let manager = SqliteConnectionManager::file("netpulse.db");
    let pool = Pool::new(manager).expect("Failed to create connection pool");

    {
        let conn = pool.get().expect("Failed to get DB connection");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS measurements (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                value REAL NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        ).expect("Failed to create measurements table");
    }

    let state = Arc::new(AppState { db: pool });

    // Clone pool for background task
    let pool = state.db.clone();
    tokio::spawn(async move {
        loop {
            if let Ok(conn) = pool.get() {
                if let Ok(bandwidth) = measure_bandwidth().await {
                    let _ = conn.execute(
                        "INSERT INTO measurements (value, timestamp) VALUES (?1, datetime('now'))",
                        &[&bandwidth],
                    );
                }
            }
            sleep(Duration::from_secs(60)).await; // measure every 60 seconds
        }
    });

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