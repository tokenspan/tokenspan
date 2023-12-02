use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use axum_macros::debug_handler;

use crate::api::task::handler::execute_task_v1;
use crate::extractor::versioning::Version;
use crate::state::AppState;

#[debug_handler]
async fn execute_task(version: Version) -> impl IntoResponse {
    match version {
        Version::V1 => execute_task_v1().await,
    }
}

pub struct TaskRouter;

impl TaskRouter {
    pub fn new() -> Router<AppState> {
        Router::new().route("/execute", post(execute_task))
    }
}
