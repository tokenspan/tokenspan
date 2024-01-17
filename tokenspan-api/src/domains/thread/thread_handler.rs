use axum::extract::State;
use axum::routing::post;
use axum::{Extension, Json, Router};
use tracing::info;

use crate::domains::models::{Execution, ParsedToken};
use crate::domains::services::ThreadServiceDyn;
use crate::domains::thread::dto::ThreadExecuteInput;
use crate::domains::thread::handler::execute_thread_v1;
use crate::domains::thread::thread_error::ThreadError;
use crate::extractors::valid_json::ValidJson;
use crate::extractors::versioning::Version;
use crate::state::AppState;

async fn execute_thread(
    version: Version,
    State(thread_service): State<ThreadServiceDyn>,
    Extension(token): Extension<Option<ParsedToken>>,
    ValidJson(input): ValidJson<ThreadExecuteInput>,
) -> anyhow::Result<Json<Execution>, ThreadError> {
    info!("{:?}", input);
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
