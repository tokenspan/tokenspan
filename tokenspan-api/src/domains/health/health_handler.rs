use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;

use crate::state::AppState;

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub struct HealthRouter;

impl HealthRouter {
    pub fn new() -> Router<AppState> {
        Router::new().route("/health", axum::routing::get(health_check))
    }
}
