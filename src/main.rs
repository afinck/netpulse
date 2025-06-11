// filepath: /netpulse/netpulse/src/main.rs
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

mod db;
mod measurements;
mod pdf_export;
mod utils;
mod web;

use web::handlers::{AppState, create_routes};

/* async fn measure_bandwidth() -> Result<f64, reqwest::Error> {
    let url = "http://proof.ovh.net/files/512MB.dat"; // 1GB test file
    let client = Client::new();
    let start = std::time::Instant::now();
    let resp = client.get(url).send().await?;
    let bytes = resp.bytes().await?.len();
    let elapsed = start.elapsed().as_secs_f64();
    println!("Downloaded {} bytes in {:.2} seconds", bytes, elapsed);
    let mbits = (bytes as f64 * 8.0) / 1_000_000.0;
    Ok(mbits / elapsed) // Mbit/s
} */

fn measure_bandwidth_speedtest() -> Option<f64> {
    let output = std::process::Command::new("speedtest")
        .arg("--accept-license")
        .arg("--accept-gdpr")
        .arg("--format=json")
        .output()
        .ok()?;
    if !output.status.success() {
        eprintln!("Speedtest CLI failed: {:?}", output);
        return None;
    }
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    let bps = json.get("download")?.get("bandwidth")?.as_f64()?;
    Some(bps / 1_000_000.0) // Convert to Mbit/s
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
                let bandwidth = tokio::task::spawn_blocking(|| measure_bandwidth_speedtest())
                    .await
                    .unwrap_or(None);
                if let Some(bandwidth) = bandwidth {
                    match conn.execute(
                        "INSERT INTO measurements (value, timestamp) VALUES (?1, datetime('now'))",
                        &[&bandwidth],
                    ) {
                        Ok(rows) => println!("Inserted measurement: {} Mbit/s (rows affected: {})", bandwidth, rows),
                        Err(e) => eprintln!("DB insert error: {}", e),
                    }
                } else {
                    eprintln!("Speedtest failed or returned no result");
                }
            } else {
                eprintln!("Failed to get DB connection from pool");
            }
            sleep(Duration::from_secs(3600)).await; // Run every hour
        }
    });

    // Router with state
    let app = create_routes(state);

    // Address festlegen
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);

    // Server starten
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}