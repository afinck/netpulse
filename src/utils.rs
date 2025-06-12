// filepath: /netpulse/netpulse/src/utils.rs
use std::fmt;
use chrono::{NaiveDateTime, Local, TimeZone};

// Custom error type for the application
#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

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
#[allow(dead_code)]
pub fn handle_error<E: std::error::Error>(err: E) -> AppError {
    AppError {
        message: err.to_string(),
    }
}