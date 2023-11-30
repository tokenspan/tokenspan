use async_graphql::dataloader::DataLoader;
use async_graphql::extensions::Tracing;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use axum::extract::Host;
use axum::http::{HeaderMap};
use axum::response::{IntoResponse, Redirect};
use axum::Extension;

use crate::api::models::ParsedToken;
use crate::api::{MutationRoot, QueryRoot};
use crate::loader::AppLoader;
use crate::state::AppState;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn build_schema(app_state: AppState) -> AppSchema {
    let loader = DataLoader::new(AppLoader::from(app_state.clone()), tokio::spawn);

    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .extension(Tracing)
    .data(app_state.db)
    .data(app_state.user_service)
    .data(app_state.auth_service)
    .data(app_state.api_key_service)
    .data(app_state.provider_service)
    .data(app_state.model_service)
    .data(app_state.parameter_service)
    .data(app_state.task_version_service)
    .data(app_state.task_service)
    .data(app_state.view_service)
    .data(app_state.execution_history_service)
    .data(loader)
    .finish()
}

pub async fn graphql_sandbox(Host(hostname): Host) -> impl IntoResponse {
    let endpoint = if hostname.contains("localhost") {
        format!("http://{}graphql", hostname)
    } else {
        format!("https://{}graphql", hostname)
    };
    Redirect::temporary(
        format!(
            "https://studio.apollographql.com/sandbox/explorer?endpoint={}",
            endpoint
        )
        .as_str(),
    )
}

pub async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    Extension(token): Extension<Option<ParsedToken>>,
    Extension(headers): Extension<HeaderMap>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let execute = schema
        .execute(req.into_inner().data(headers).data(token))
        .await;

    execute.into()
}
