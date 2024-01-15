use anyhow::Result;
use dojo_orm::Database;
use magic_crypt::new_magic_crypt;
use std::ops::DerefMut;

use crate::api::services::*;
use crate::configs::AppConfig;

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!("./migrations");
}

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserServiceDyn,
    pub auth_service: AuthServiceDyn,
    pub api_key_service: ApiKeyServiceDyn,
    pub provider_service: ProviderServiceDyn,
    pub model_service: ModelServiceDyn,
    pub thread_version_service: ThreadVersionServiceDyn,
    pub thread_service: ThreadServiceDyn,
    pub execution_service: ExecutionServiceDyn,
    pub parameter_service: ParameterServiceDyn,
    pub message_service: MessageServiceDyn,
    pub function_service: FunctionServiceDyn,
}

impl AppState {
    pub async fn new(app_config: &AppConfig) -> Result<Self> {
        let mc = new_magic_crypt!(app_config.encryption.secret.clone(), 256);
        let db = Database::new(app_config.database.url.as_str()).await?;
        let mut conn = db.get().await?;
        let client = conn.deref_mut();
        embedded::migrations::runner()
            .run_async(client)
            .await
            .unwrap();

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

        let provider_service: ProviderServiceDyn =
            ProviderService::builder().db(db.clone()).build().into();
        let model_service: ModelServiceDyn = ModelService::builder().db(db.clone()).build().into();

        let message_service: MessageServiceDyn =
            MessageService::builder().db(db.clone()).build().into();

        let execution_service: ExecutionServiceDyn =
            ExecutionService::builder().db(db.clone()).build().into();

        let parameter_service: ParameterServiceDyn =
            ParameterService::builder().db(db.clone()).build().into();

        let thread_version_service: ThreadVersionServiceDyn = ThreadVersionService::builder()
            .db(db.clone())
            .parameter_service(parameter_service.clone())
            .message_service(message_service.clone())
            .build()
            .into();

        let thread_service: ThreadServiceDyn = ThreadService::builder()
            .db(db.clone())
            .api_key_service(api_key_service.clone())
            .model_service(model_service.clone())
            .execution_service(execution_service.clone())
            .thread_version_service(thread_version_service.clone())
            .parameter_service(parameter_service.clone())
            .message_service(message_service.clone())
            .provider_service(provider_service.clone())
            .build()
            .into();

        let function_service: FunctionServiceDyn =
            FunctionService::builder().db(db.clone()).build().into();

        Ok(Self {
            user_service,
            auth_service,
            api_key_service,
            provider_service,
            model_service,
            thread_version_service,
            thread_service,
            execution_service,
            parameter_service,
            message_service,
            function_service,
        })
    }
}
