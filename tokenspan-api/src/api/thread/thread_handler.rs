use axum::extract::State;
use axum::routing::post;
use axum::{Extension, Json, Router};

use crate::api::models::{Execution, ParsedToken};
use crate::api::services::ThreadServiceDyn;
use crate::api::thread::dto::ThreadExecuteInput;
use crate::api::thread::handler::execute_thread_v1;
use crate::api::thread::thread_error::ThreadError;
use crate::extractor::valid_json::ValidJson;
use crate::extractor::versioning::Version;
use crate::state::AppState;

async fn execute_thread(
    version: Version,
    State(thread_service): State<ThreadServiceDyn>,
    Extension(token): Extension<Option<ParsedToken>>,
    ValidJson(input): ValidJson<ThreadExecuteInput>,
) -> anyhow::Result<Json<Execution>, ThreadError> {
    match version {
        Version::V1 => execute_thread_v1(thread_service, input, token).await,
    }
}

pub struct ThreadRouter;

impl ThreadRouter {
    pub fn new() -> Router<AppState> {
        Router::new().route("/execute", post(execute_thread))
    }
}
