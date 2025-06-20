// filepath: /netpulse/netpulse/src/measurements.rs
use chrono::Local;
use r2d2_sqlite::rusqlite::{params, Connection};
use rusqlite::params_from_iter;
use serde::Serialize;

#[derive(Serialize)]
pub struct Measurement {
    pub id: i32,
    pub value: f64,
    pub timestamp: String,
}

pub fn get_measurements(
    conn: &Connection,
    range: &str,
) -> Result<Vec<Measurement>, rusqlite::Error> {
    let (query, params): (&str, Vec<&dyn rusqlite::ToSql>) = match range {
        "day" => (
            "SELECT id, value, timestamp FROM measurements WHERE date(timestamp) = date('now') ORDER BY timestamp ASC",
            Vec::<&dyn rusqlite::ToSql>::new()
        ),
        // Week: one per day
        "week" => (
            "SELECT MIN(id) as id, AVG(value) as value, date(timestamp) || 'T00:00:00' as timestamp
             FROM measurements
             WHERE date(timestamp) >= date('now', '-6 days')
             GROUP BY date(timestamp)
             ORDER BY timestamp ASC",
            Vec::<&dyn rusqlite::ToSql>::new()

        ),
        // Month: one per day
        "month" => (
            "SELECT MIN(id) as id, AVG(value) as value, date(timestamp) || 'T00:00:00' as timestamp
             FROM measurements
             WHERE date(timestamp) >= date('now', 'start of month')
             GROUP BY date(timestamp)
             ORDER BY timestamp ASC",
            Vec::<&dyn rusqlite::ToSql>::new()

        ),
        // Year: one per month
        "year" => (
            "SELECT MIN(id) as id, AVG(value) as value, strftime('%Y-%m-01T00:00:00', timestamp) as timestamp
             FROM measurements
             WHERE date(timestamp) >= date('now', '-1 year')
             GROUP BY strftime('%Y-%m', timestamp)
             ORDER BY timestamp ASC",
            Vec::<&dyn rusqlite::ToSql>::new()
        ),
        _ => (
            "SELECT id, value, timestamp FROM measurements ORDER BY timestamp ASC",
            Vec::<&dyn rusqlite::ToSql>::new()
        ),
    };

    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map(params_from_iter(params), |row| {
        Ok(Measurement {
            id: row.get(0)?,
            value: row.get(1)?,
            timestamp: row.get(2)?,
        })
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
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
