use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use axum_macros::debug_handler;

use crate::api::services::TaskServiceDyn;
use crate::api::task::dto::TaskExecuteInput;
use crate::api::task::handler::execute_task_v1;
use crate::extractor::valid_json::ValidJson;
use crate::extractor::versioning::Version;
use crate::state::AppState;

async fn execute_task(
    version: Version,
    State(task_service): State<TaskServiceDyn>,
    ValidJson(input): ValidJson<TaskExecuteInput>,
) -> impl IntoResponse {
    match version {
        Version::V1 => execute_task_v1(task_service, input).await,
    }
}

pub struct TaskRouter;

impl TaskRouter {
    pub fn new() -> Router<AppState> {
        Router::new().route("/execute", post(execute_task))
    }
}
