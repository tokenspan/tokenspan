use async_graphql::{MergedObject, MergedSubscription};
use axum::Router;

use crate::state::AppState;

mod api_key;
mod auth;
mod execution;
mod health;
mod model;
mod provider;
mod task;
mod task_version;
mod user;

pub mod repositories {
    pub use super::api_key::api_key_repository::*;
    pub use super::execution::execution_repository::*;
    pub use super::model::model_repository::*;
    pub use super::provider::provider_repository::*;
    pub use super::task::task_repository::*;
    pub use super::task_version::task_version_repository::*;
    pub use super::user::user_repository::*;
}
pub mod services {
    pub use super::api_key::api_key_service::*;
    pub use super::auth::auth_service::*;
    pub use super::execution::execution_service::*;
    pub use super::model::model_service::*;
    pub use super::provider::provider_service::*;
    pub use super::task::task_service::*;
    pub use super::task_version::task_version_service::*;
    pub use super::user::user_service::*;
}

pub mod models {
    pub use super::api_key::api_key_model::*;
    pub use super::auth::auth_model::*;
    pub use super::execution::execution_model::*;
    pub use super::model::model_model::*;
    pub use super::provider::provider_model::*;
    pub use super::task::task_model::*;
    pub use super::task_version::models::*;
    pub use super::user::user_model::*;
}

pub mod types {
    pub use super::execution::execution_type::*;
    pub use super::user::user_type::*;
}

pub mod dto {
    pub use super::api_key::dto::*;
    pub use super::auth::dto::*;
    pub use super::execution::dto::*;
    pub use super::model::dto::*;
    pub use super::provider::dto::*;
    pub use super::task::dto::*;
    pub use super::task_version::dto::*;
    pub use super::user::dto::*;
}

pub mod caches {
    pub use super::api_key::api_key_cache;
    pub use super::model::model_cache;
}

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    pub auth::AuthQuery,
    pub user::UserQuery,
    pub task::TaskQuery,
    pub api_key::ApiKeyQuery,
    pub provider::ProviderQuery,
    pub model::ModelQuery,
    pub task_version::TaskVersionQuery,
    pub execution::ExecutionQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    pub auth::AuthMutation,
    pub user::UserMutation,
    pub task::TaskMutation,
    pub api_key::ApiKeyMutation,
    pub provider::ProviderMutation,
    pub model::ModelMutation,
    pub task_version::TaskVersionMutation,
);

#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(pub task::TaskSubscription);

pub struct ApiRouter;

impl ApiRouter {
    pub fn new() -> Router<AppState> {
        Router::new()
            .nest("/tasks", task::TaskRouter::new())
            .merge(health::HealthRouter::new())
    }
}
