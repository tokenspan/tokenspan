use async_graphql::dataloader::DataLoader;
use async_graphql::extensions::Tracing;
use async_graphql::http::GraphiQLSource;
use async_graphql::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Host;
use axum::response::{IntoResponse, Redirect};
use axum::{response, Extension};
use axum_extra::headers::HeaderMap;

use crate::api::loaders::{
    ApiKeyLoader, ExecutionLoader, ModelLoader, ParameterLoader, ProviderLoader, TaskLoader,
    TaskVersionLoader, UserLoader,
};
use crate::api::models::ParsedToken;
use crate::api::{MutationRoot, QueryRoot, SubscriptionRoot};
use crate::configs::AppConfig;
use crate::state::AppState;

pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub async fn build_schema(app_state: AppState) -> AppSchema {
    let api_key_loader = DataLoader::new(
        ApiKeyLoader::new(app_state.api_key_service.clone()),
        tokio::spawn,
    );
    let model_loader = DataLoader::new(
        ModelLoader::new(app_state.model_service.clone()),
        tokio::spawn,
    );
    let provider_loader = DataLoader::new(
        ProviderLoader::new(app_state.provider_service.clone()),
        tokio::spawn,
    );
    let task_loader = DataLoader::new(
        TaskLoader::new(app_state.task_service.clone()),
        tokio::spawn,
    );
    let task_version_loader = DataLoader::new(
        TaskVersionLoader::new(app_state.task_version_service.clone()),
        tokio::spawn,
    );
    let user_loader = DataLoader::new(
        UserLoader::new(app_state.user_service.clone()),
        tokio::spawn,
    );
    let parameter_loader = DataLoader::new(
        ParameterLoader::new(app_state.parameter_service.clone()),
        tokio::spawn,
    );
    let execution_loader = DataLoader::new(
        ExecutionLoader::new(app_state.execution_service.clone()),
        tokio::spawn,
    );
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        SubscriptionRoot::default(),
    )
    .extension(Tracing)
    .data(app_state.user_service)
    .data(app_state.auth_service)
    .data(app_state.api_key_service)
    .data(app_state.provider_service)
    .data(app_state.model_service)
    .data(app_state.task_version_service)
    .data(app_state.task_service)
    .data(app_state.execution_service)
    .data(app_state.parameter_service)
    .data(app_state.message_service)
    .data(api_key_loader)
    .data(model_loader)
    .data(provider_loader)
    .data(task_loader)
    .data(task_version_loader)
    .data(user_loader)
    .data(parameter_loader)
    .data(execution_loader)
    .finish()
}
pub async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

pub async fn graphql_sandbox(Host(hostname): Host) -> impl IntoResponse {
    let endpoint = if hostname.contains("localhost") {
        format!("http://{}/graphql", hostname)
    } else {
        format!("https://{}/graphql", hostname)
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
    Extension(config): Extension<AppConfig>,
    Extension(token): Extension<Option<ParsedToken>>,
    Extension(headers): Extension<HeaderMap>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let execute = schema
        .execute(req.into_inner().data(headers).data(token).data(config))
        .await;

    execute.into()
}
