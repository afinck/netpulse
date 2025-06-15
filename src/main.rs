// filepath: /netpulse/netpulse/src/main.rs
use log::LevelFilter;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use syslog::{Facility, Formatter3164, BasicLogger};
use tokio::time::interval;

mod measurements;
mod pdf_export;
mod utils;
mod web;

use web::handlers::{create_routes, AppState};

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
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "netpulse".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).unwrap_or_else(|e| {
        eprintln!("Syslog not available: {e}");
        std::process::exit(1);
    });
    let logger = BasicLogger::new(logger);
    log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(LevelFilter::Info)).unwrap();

    log::info!("NetPulse started"); // This should go to syslog

    let db_path = std::env::var("NETPULSE_DB_PATH").unwrap_or_else(|_| "/var/lib/netpulse/netpulse.db".to_string());

    // Create a connection manager for SQLite
    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::new(manager).unwrap_or_else(|e| {
        crate::utils::handle_error(&e);
        std::process::exit(1);
    });

    {
        let conn = pool.get().unwrap_or_else(|e| {
            crate::utils::handle_error(&e);
            std::process::exit(1);
        });
        if let Err(e) = conn.execute(
            "CREATE TABLE IF NOT EXISTS measurements (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                value REAL NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        ) {
            crate::utils::handle_error(&e);
            std::process::exit(1);
        }

        if let Err(e) = conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_measurements_timestamp ON measurements(timestamp);",
            [],
        ) {
            crate::utils::handle_error(&e);
            std::process::exit(1);
        }
    }

    let state = Arc::new(AppState { db: pool });

    // Clone your DB pool or app state as needed
    let db_pool = Arc::new(state.db.clone());

    // Spawn periodic speedtest task
    let db_pool_clone = db_pool.clone();
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(1800)); // 30 minutes
        loop {
            ticker.tick().await;
            if let Some(bandwidth) = measure_bandwidth_speedtest() {
                let conn = match db_pool_clone.get() {
                    Ok(conn) => conn,
                    Err(e) => {
                        crate::utils::handle_error(&e);
                        continue;
                    }
                };
                if let Err(e) = crate::measurements::insert_measurement(&conn, bandwidth) {
                    crate::utils::handle_error(&e);
                } else {
                    log::info!("Speedtest result saved: {} Mbit/s", bandwidth);
                }
            } else {
                crate::utils::handle_error(&std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Speedtest failed",
                ));
            }
        }
    });

    // Router with state
    let app = create_routes(state);

    // Address festlegen
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);

    // Server starten
    if let Err(e) = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        crate::utils::handle_error(&e);
    }
}
