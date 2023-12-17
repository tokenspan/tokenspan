use anyhow::Result;
use sea_orm::DatabaseConnection;

use crate::api::caches::api_key_cache::{ApiKeyCache, ApiKeyCacheDyn};
use crate::api::caches::model_cache::{ModelCache, ModelCacheDyn};
use crate::api::services::*;
use crate::configs::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserServiceDyn,
    pub auth_service: AuthServiceDyn,
    pub api_key_service: ApiKeyServiceDyn,
    pub parameter_service: ParameterServiceDyn,
    pub provider_service: ProviderServiceDyn,
    pub model_service: ModelServiceDyn,
    pub task_version_service: TaskVersionServiceDyn,
    pub task_service: TaskServiceDyn,
    pub execution_service: ExecutionServiceDyn,
    pub message_service: MessageServiceDyn,

    pub api_key_cache: ApiKeyCacheDyn,
    pub model_cache: ModelCacheDyn,
}

impl AppState {
    pub async fn new(db: DatabaseConnection, app_config: AppConfig) -> Result<Self> {
        let user_service: UserServiceDyn = UserService::new(db.clone()).into();
        let auth_service: AuthServiceDyn =
            AuthService::new(user_service.clone(), app_config.auth.clone()).into();

        let api_key_service: ApiKeyServiceDyn =
            ApiKeyService::new(db.clone(), app_config.encryption.clone()).into();

        let api_key_cache: ApiKeyCacheDyn = ApiKeyCache::new(api_key_service.clone()).await?.into();

        let model_service: ModelServiceDyn = ModelService::new(db.clone()).into();
        let model_cache: ModelCacheDyn = ModelCache::new(model_service.clone()).await?.into();

        let provider_service: ProviderServiceDyn = ProviderService::new(db.clone()).into();

        let execution_service: ExecutionServiceDyn = ExecutionService::new(db.clone()).into();

        let parameter_service: ParameterServiceDyn = ParameterService::new(db.clone()).into();
        let message_service: MessageServiceDyn = MessageService::new(db.clone()).into();

        let task_version_service: TaskVersionServiceDyn = TaskVersionService::builder()
            .db(db.clone())
            .parameter_service(parameter_service.clone())
            .message_service(message_service.clone())
            .build()
            .into();

        let task_service: TaskServiceDyn = TaskService::builder()
            .db(db.clone())
            .api_key_cache(api_key_cache.clone())
            .model_cache(model_cache.clone())
            .execution_service(execution_service.clone())
            .task_version_service(task_version_service.clone())
            .parameter_service(parameter_service.clone())
            .message_service(message_service.clone())
            .build()
            .into();

        Ok(Self {
            user_service,
            auth_service,
            api_key_service,
            provider_service,
            model_service,
            task_version_service,
            task_service,
            execution_service,
            parameter_service,
            message_service,

            api_key_cache,
            model_cache,
        })
    }
}
