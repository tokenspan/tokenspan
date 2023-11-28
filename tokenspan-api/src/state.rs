use crate::api::services::*;
use crate::prisma;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<prisma::PrismaClient>,
    pub user_service: UserServiceDyn,
    pub auth_service: AuthServiceDyn,
    pub api_key_service: ApiKeyServiceDyn,
    pub provider_service: ProviderServiceDyn,
    pub model_service: ModelServiceDyn,
    pub parameter_service: ParameterServiceDyn,
    pub task_version_service: TaskVersionServiceDyn,
    pub task_service: TaskServiceDyn,
    pub view_service: ViewServiceDyn,
    pub execution_history_service: ExecutionHistoryServiceDyn,
}

impl AppState {
    pub async fn init() -> Self {
        let mongo_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = prisma::new_client_with_url(mongo_url.as_str())
            .await
            .expect("Failed to connect to Prisma client");
        let db = Arc::new(db);

        let user_service: UserServiceDyn = UserService::new(db.clone()).into();
        let auth_service: AuthServiceDyn = AuthService::new(user_service.clone()).into();
        let api_key_service: ApiKeyServiceDyn = ApiKeyService::new(db.clone()).into();
        let provider_service: ProviderServiceDyn = ProviderService::new(db.clone()).into();
        let model_service: ModelServiceDyn = ModelService::new(db.clone()).into();
        let parameter_service: ParameterServiceDyn = ParameterService::new(db.clone()).into();
        let task_version_service: TaskVersionServiceDyn =
            TaskVersionService::new(db.clone()).into();
        let view_service: ViewServiceDyn = ViewService::new(db.clone()).into();
        let execution_history_service: ExecutionHistoryServiceDyn =
            ExecutionHistoryService::new(db.clone()).into();
        let task_service: TaskServiceDyn = TaskService::new(
            db.clone(),
            parameter_service.clone(),
            model_service.clone(),
            api_key_service.clone(),
            execution_history_service.clone(),
        )
        .into();

        Self {
            db,
            user_service,
            auth_service,
            api_key_service,
            provider_service,
            model_service,
            parameter_service,
            task_version_service,
            task_service,
            view_service,
            execution_history_service,
        }
    }
}
