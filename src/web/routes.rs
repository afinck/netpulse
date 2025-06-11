// filepath: /netpulse/netpulse/src/web/routes.rs
use axum::{
    routing::{get, post},
    Router,
};

use crate::web::handlers::{dashboard_handler, measurements_handler};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(dashboard_handler))
        .route("/measurements", post(measurements_handler))
}