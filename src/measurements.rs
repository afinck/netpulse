// filepath: /netpulse/netpulse/src/measurements.rs
use rusqlite::{Connection, Result};
use r2d2_sqlite::rusqlite::params;
use chrono::Local;

#[derive(serde::Serialize)]
pub struct Measurement {
    pub id: i32,
    pub value: f64,
    pub timestamp: String,
}

pub fn get_measurements(conn: &Connection) -> Result<Vec<Measurement>> {
    let mut stmt = conn.prepare("SELECT id, value, timestamp FROM measurements")?;
    let measurement_iter = stmt.query_map([], |row| {
        Ok(Measurement {
            id: row.get(0)?,
            value: row.get(1)?,
            timestamp: row.get(2)?,
        })
    })?;

    let mut measurements = Vec::new();
    for measurement in measurement_iter {
        measurements.push(measurement?);
    }
    Ok(measurements)
}

/// Inserts a new measurement into the database.
/// Returns Ok(()) on success, Err otherwise.
pub fn insert_measurement(conn: &Connection, value: f64) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO measurements (value, timestamp) VALUES (?1, ?2)",
        params![value, timestamp],
    )?;
    Ok(())
}