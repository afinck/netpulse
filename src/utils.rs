// filepath: /netpulse/netpulse/src/utils.rs
use chrono::NaiveDateTime;

// Utility function for formatting dates
pub fn format_date(date: &str) -> String {
    // Try to parse the input date string as a NaiveDateTime
    if let Ok(dt) = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S") {
        // Output ISO 8601 for Chart.js compatibility
        return dt.format("%Y-%m-%dT%H:%M:%S").to_string();
    }
    // Fallback: return as-is
    date.to_string()
}

// Utility function for handling errors
pub fn handle_error<E: std::fmt::Display>(err: E) {
    eprintln!("Error: {}", err);
}
