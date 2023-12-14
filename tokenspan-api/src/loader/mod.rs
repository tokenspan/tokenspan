use crate::api::services::*;
use crate::state::AppState;

pub struct AppLoader {
    pub user_service: UserServiceDyn,
    pub provider_service: ProviderServiceDyn,
    pub model_service: ModelServiceDyn,
    pub task_version_service: TaskVersionServiceDyn,
    pub task_service: TaskServiceDyn,
    pub api_key_service: ApiKeyServiceDyn,
    pub execution_service: ExecutionServiceDyn,
}

impl From<AppState> for AppLoader {
    fn from(state: AppState) -> Self {
        Self {
            user_service: state.user_service,
            provider_service: state.provider_service,
            model_service: state.model_service,
            task_version_service: state.task_version_service,
            task_service: state.task_service,
            api_key_service: state.api_key_service,
            execution_service: state.execution_service,
        }
    }
}
