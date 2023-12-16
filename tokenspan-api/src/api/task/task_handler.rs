use axum::extract::State;
use axum::routing::post;
use axum::{Extension, Json, Router};

use crate::api::models::ParsedToken;
use crate::api::services::TaskServiceDyn;
use crate::api::task::dto::TaskExecuteInput;
use crate::api::task::handler::execute_task_v1;
use crate::api::task::task_error::TaskError;
use crate::extractor::valid_json::ValidJson;
use crate::extractor::versioning::Version;
use crate::state::AppState;

async fn execute_task(
    version: Version,
    State(task_service): State<TaskServiceDyn>,
    Extension(token): Extension<Option<ParsedToken>>,
    ValidJson(input): ValidJson<TaskExecuteInput>,
) -> anyhow::Result<Json<serde_json::Value>, TaskError> {
    match version {
        Version::V1 => execute_task_v1(task_service, input, token).await,
    }
}

pub struct TaskRouter;

impl TaskRouter {
    pub fn new() -> Router<AppState> {
        Router::new().route("/execute", post(execute_task))
    }
}
