// filepath: /netpulse/netpulse/src/web/handlers.rs
use axum::{
    extract::{Extension, Json},
    response::{Html, IntoResponse, Response},
    routing::get,
    http::header,
    Router,
};
use std::sync::Arc;
use crate::utils::format_date;
use crate::measurements::get_measurements;
use crate::pdf_export::export_to_pdf;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tower_http::services::ServeDir;

/// Shared application state, e.g. database connection pool
pub struct AppState {
    pub db: Pool<SqliteConnectionManager>,
}

pub async fn dashboard_handler() -> Html<String> {
    Html(include_str!("../../templates/dashboard.html").to_string())
}

pub async fn measurements_handler(
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let conn = state.db.get().unwrap();
    match get_measurements(&conn) {
        Ok(measurements) => {
            // Map each measurement, formatting the timestamp
            let formatted: Vec<_> = measurements.into_iter().map(|m| {
                serde_json::json!({
                    "id": m.id,
                    "value": m.value,
                    "timestamp": format_date(&m.timestamp), // 24h format
                })
            }).collect();
            Json(formatted)
        },
        Err(_) => Json(vec![]),
    }
}

pub async fn pdf_export_handler(
    Extension(state): Extension<Arc<AppState>>,
) -> Response {
    let conn = state.db.get().unwrap();
    let data = get_measurements(&conn).unwrap_or_default();
    let json = serde_json::to_string(&data).unwrap_or_default();
    let file_path = "/tmp/export.pdf";
    let pdf_data = export_to_pdf(&json, file_path).unwrap_or_default();
    (
        [(header::CONTENT_TYPE, "application/pdf")],
        pdf_data
    ).into_response()
}

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(dashboard_handler))
        .route("/measurements", get(measurements_handler))
        .route("/export/pdf", get(pdf_export_handler))
        .nest_service("/static", ServeDir::new("static"))
        .layer(Extension(state))
}