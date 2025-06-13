use rusqlite::{Connection, params};
use chrono::{Duration, Local, Timelike};

fn main() -> rusqlite::Result<()> {
    // Adjust the path to your database as needed
    let db_path = "netpulse.db";
    let conn = Connection::open(db_path)?;

    // Create table if not exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS measurements (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            value REAL NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;

    // Remove old test data (optional)
    conn.execute("DELETE FROM measurements", [])?;

    let now = Local::now().naive_local();

    // Insert 12 measurements per day for the last 1 year (365 days)
    for days_ago in 0..730 {
        let date = now - Duration::days(days_ago);
        // 12 measurements per day, every 2 hours
        for hour in (0..24).step_by(2) {
            let dt = date
                .with_hour(hour)
                .unwrap_or(date)
                .with_minute(0)
                .unwrap_or(date)
                .with_second(0)
                .unwrap_or(date);
            let value = 50.0 + (days_ago as f64 % 20.0) + (hour as f64 / 10.0);
            conn.execute(
                "INSERT INTO measurements (value, timestamp) VALUES (?1, ?2)",
                params![value, dt.format("%Y-%m-%d %H:%M:%S").to_string()],
            )?;
        }
    }

    println!("Test data generated: 12 measurements per day for the last 1 year in {db_path}");
    Ok(())
}