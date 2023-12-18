use anyhow::Result;
use magic_crypt::new_magic_crypt;
use sea_orm::DatabaseConnection;

use crate::api::services::*;
use crate::configs::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserServiceDyn,
    pub auth_service: AuthServiceDyn,
    pub api_key_service: ApiKeyServiceDyn,
    pub provider_service: ProviderServiceDyn,
    pub model_service: ModelServiceDyn,
    pub task_version_service: TaskVersionServiceDyn,
    pub task_service: TaskServiceDyn,
    pub execution_service: ExecutionServiceDyn,
    pub parameter_service: ParameterServiceDyn,
}

impl AppState {
    pub async fn new(db: DatabaseConnection, app_config: &AppConfig) -> Result<Self> {
        let mc = new_magic_crypt!(app_config.encryption.secret.clone(), 256);

        let user_service: UserServiceDyn = UserService::builder().db(db.clone()).build().into();
        let auth_service: AuthServiceDyn = AuthService::builder()
            .user_service(user_service.clone())
            .auth_config(app_config.auth.clone())
            .build()
            .into();

        let api_key_service: ApiKeyServiceDyn = ApiKeyService::builder()
            .db(db.clone())
            .mc(mc)
            .build()
            .into();

        let model_service: ModelServiceDyn = ModelService::builder().db(db.clone()).build().into();

        let provider_service: ProviderServiceDyn =
            ProviderService::builder().db(db.clone()).build().into();

        let execution_service: ExecutionServiceDyn =
            ExecutionService::builder().db(db.clone()).build().into();

        let task_version_service: TaskVersionServiceDyn =
            TaskVersionService::builder().db(db.clone()).build().into();

        let parameter_service: ParameterServiceDyn =
            ParameterService::builder().db(db.clone()).build().into();

        let task_service: TaskServiceDyn = TaskService::builder()
            .db(db.clone())
            .api_key_service(api_key_service.clone())
            .model_service(model_service.clone())
            .execution_service(execution_service.clone())
            .task_version_service(task_version_service.clone())
            .parameter_service(parameter_service.clone())
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
        })
    }
}
