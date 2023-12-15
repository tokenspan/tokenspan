use std::sync::Arc;

use crate::api::caches::api_key_cache::{ApiKeyCache, ApiKeyCacheDyn};
use crate::api::caches::model_cache::{ModelCache, ModelCacheDyn};
use crate::api::services::*;
use crate::configs::AppConfig;
use crate::repository::RootRepository;

#[derive(Clone)]
pub struct AppState {
    pub repository: Arc<RootRepository>,
    pub user_service: UserServiceDyn,
    pub auth_service: AuthServiceDyn,
    pub api_key_service: ApiKeyServiceDyn,
    pub provider_service: ProviderServiceDyn,
    pub model_service: ModelServiceDyn,
    pub task_version_service: TaskVersionServiceDyn,
    pub task_service: TaskServiceDyn,
    pub execution_service: ExecutionServiceDyn,

    pub api_key_cache: ApiKeyCacheDyn,
    pub model_cache: ModelCacheDyn,
}

impl AppState {
    pub async fn new(app_config: Arc<AppConfig>) -> Self {
        let url = app_config.database.url.clone();

        let repository = RootRepository::new_with_uri(url).await;
        let repository = Arc::new(repository);

        let user_service: UserServiceDyn = UserService::new(repository.clone()).into();
        let auth_service: AuthServiceDyn =
            AuthService::new(user_service.clone(), app_config.auth.clone()).into();

        let api_key_service: ApiKeyServiceDyn =
            ApiKeyService::new(repository.clone(), app_config.encryption.clone()).into();

        let api_key_cache = ApiKeyCache::new(api_key_service.clone()).await.unwrap();
        let api_key_cache = Arc::new(api_key_cache);

        let model_service: ModelServiceDyn = ModelService::new(repository.clone()).into();
        let model_cache = ModelCache::new(model_service.clone()).await.unwrap();
        let model_cache = Arc::new(model_cache);

        let provider_service: ProviderServiceDyn = ProviderService::new(repository.clone()).into();
        let task_version_service: TaskVersionServiceDyn =
            TaskVersionService::new(repository.clone()).into();
        let execution_service: ExecutionServiceDyn =
            ExecutionService::new(repository.clone()).into();
        let task_service: TaskServiceDyn = TaskService::new(
            repository.clone(),
            api_key_cache.clone(),
            model_cache.clone(),
            execution_service.clone(),
            task_version_service.clone(),
        )
        .into();

        Self {
            repository,
            user_service,
            auth_service,
            api_key_service,
            provider_service,
            model_service,
            task_version_service,
            task_service,
            execution_service,

            api_key_cache,
            model_cache,
        }
    }
}
