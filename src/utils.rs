// filepath: /netpulse/netpulse/src/utils.rs
use std::fmt;

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
    // Placeholder for date formatting logic
    date.to_string()
}

// Utility function for handling errors
pub fn handle_error<E: std::error::Error>(err: E) -> AppError {
    AppError {
        message: err.to_string(),
    }
}