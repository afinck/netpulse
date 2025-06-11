// filepath: /netpulse/netpulse/src/measurements.rs
use rusqlite::{params, Connection, Result};

#[derive(serde::Serialize)]
pub struct Measurement {
    pub id: i32,
    pub value: f64,
    pub timestamp: String,
}

pub fn create_measurement(conn: &Connection, value: f64, timestamp: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO measurements (value, timestamp) VALUES (?1, ?2)",
        params![value, timestamp],
    )?;
    Ok(())
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