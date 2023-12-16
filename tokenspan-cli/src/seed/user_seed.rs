use async_trait::async_trait;

use serde::Deserialize;
use tokio_stream::StreamExt;

use crate::seed::Seed;
use tokenspan_api::api::types::Role;
use tokenspan_api::configs::AppConfig;
use tokenspan_api::state::AppState;

#[derive(Debug, Deserialize, Clone)]
pub struct UserRef {
    pub email: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: Role,
}

pub struct UserSeed {
    pub data: Vec<User>,
    pub config: AppConfig,
    pub state: AppState,
}

#[async_trait]
impl Seed for UserSeed {
    async fn new(config: AppConfig, state: AppState) -> anyhow::Result<Self> {
        let data = Self::load().await?;
        Ok(Self {
            data,
            config,
            state,
        })
    }

    async fn save(&self) -> anyhow::Result<()> {
        let user_service = self.state.user_service.clone();
        let mut stream = tokio_stream::iter(self.data.clone());
        while let Some(user) = stream.next().await {
            let result = user_service.find_by_email(user.email.clone()).await?;
            if let Some(user) = result {
                println!("User: {} already existed", user.email);
                continue;
            }

            let user = user_service
                .create_with_role(user.email, user.username, user.password, user.role)
                .await?;
            println!("User: {} created", user.email)
        }

        Ok(())
    }

    fn path() -> &'static str {
        "./seed/users"
    }
}
