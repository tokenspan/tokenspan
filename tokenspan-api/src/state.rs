use crate::api::services::*;
use crate::repository::RootRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub repository: Arc<RootRepository>,
    pub user_service: UserServiceDyn,
    pub auth_service: AuthServiceDyn,
    pub api_key_service: ApiKeyServiceDyn,
    pub provider_service: ProviderServiceDyn,
    pub model_service: ModelServiceDyn,
    pub parameter_service: ParameterServiceDyn,
    pub task_version_service: TaskVersionServiceDyn,
    pub task_service: TaskServiceDyn,
    pub view_service: ViewServiceDyn,
    pub execution_service: ExecutionServiceDyn,
}

impl AppState {
    pub async fn init() -> Self {
        let mongo_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let repository = RootRepository::new_with_uri(mongo_url).await;
        let repository = Arc::new(repository);

        let user_service: UserServiceDyn = UserService::new(repository.clone()).into();
        let auth_service: AuthServiceDyn = AuthService::new(user_service.clone()).into();
        let api_key_service: ApiKeyServiceDyn = ApiKeyService::new(repository.clone()).into();
        let provider_service: ProviderServiceDyn = ProviderService::new(repository.clone()).into();
        let model_service: ModelServiceDyn = ModelService::new(repository.clone()).into();
        let parameter_service: ParameterServiceDyn =
            ParameterService::new(repository.clone()).into();
        let task_version_service: TaskVersionServiceDyn =
            TaskVersionService::new(repository.clone()).into();
        let view_service: ViewServiceDyn = ViewService::new(repository.clone()).into();
        let execution_service: ExecutionServiceDyn =
            ExecutionService::new(repository.clone()).into();
        let task_service: TaskServiceDyn = TaskService::new(
            repository.clone(),
            parameter_service.clone(),
            model_service.clone(),
            api_key_service.clone(),
            execution_service.clone(),
        )
        .into();

        Self {
            repository,
            user_service,
            auth_service,
            api_key_service,
            provider_service,
            model_service,
            parameter_service,
            task_version_service,
            task_service,
            view_service,
            execution_service,
        }
    }
}
