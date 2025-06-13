// filepath: /netpulse/netpulse/src/web/handlers.rs
use crate::measurements::get_measurements;
use crate::pdf_export::convert_json_to_pdf;
use crate::utils::{format_date, handle_error};
use axum::{
    extract::{Extension, Json, Query},
    http::header,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::collections::HashMap;
use std::sync::Arc;
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
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let conn = match state.db.get() {
        Ok(c) => c,
        Err(e) => {
            handle_error(&e);
            return Json(vec![]);
        }
    };
    let range = params.get("range").map(|s| s.as_str()).unwrap_or("day");
    match get_measurements(&conn, range) {
        Ok(measurements) => {
            // Map each measurement, formatting the timestamp
            let formatted: Vec<_> = measurements
                .into_iter()
                .map(|m| {
                    serde_json::json!({
                        "id": m.id,
                        "value": m.value,
                        "timestamp": format_date(&m.timestamp), // 24h format
                    })
                })
                .collect();
            Json(formatted)
        }
        Err(e) => {
            handle_error(&e);
            Json(vec![])
        }
    }
}

pub async fn pdf_export_handler(Extension(state): Extension<Arc<AppState>>) -> Response {
    let conn = match state.db.get() {
        Ok(c) => c,
        Err(e) => {
            handle_error(&e);
            return ([(header::CONTENT_TYPE, "application/pdf")], vec![]).into_response();
        }
    };
    let data = match get_measurements(&conn, "day") {
        Ok(d) => d,
        Err(e) => {
            handle_error(&e);
            vec![]
        }
    };
    let json = match serde_json::to_string(&data) {
        Ok(j) => j,
        Err(e) => {
            handle_error(&e);
            String::new()
        }
    };
    let file_path = "/tmp/export.pdf";
    let pdf_data = convert_json_to_pdf(&json, file_path);
    ([(header::CONTENT_TYPE, "application/pdf")], pdf_data).into_response()
}

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(dashboard_handler))
        .route("/measurements", get(measurements_handler))
        .route("/export/pdf", get(pdf_export_handler))
        .nest_service("/static", ServeDir::new("static"))
        .layer(Extension(state))
}
