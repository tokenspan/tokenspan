use async_trait::async_trait;
use serde::Deserialize;
use tokio_stream::StreamExt;

use tokenspan_api::api::models::UserRole;
use tokenspan_api::configs::AppConfig;
use tokenspan_api::state::AppState;
use tracing::{info, warn};

use crate::seed::Seed;

#[derive(Debug, Deserialize, Clone)]
pub struct UserRef {
    pub email: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: UserRole,
}

pub struct UserSeed {
    pub data: Vec<User>,
    pub config: AppConfig,
    pub state: AppState,
}

impl UserSeed {
    pub async fn new(config: AppConfig, state: AppState) -> anyhow::Result<Self> {
        let data = Self::load().await?;
        Ok(Self {
            data,
            config,
            state,
        })
    }

    pub async fn new_with_data(
        config: AppConfig,
        state: AppState,
        data: Vec<User>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            data,
            config,
            state,
        })
    }
}

#[async_trait]
impl Seed for UserSeed {
    async fn save(&self) -> anyhow::Result<()> {
        let user_service = self.state.user_service.clone();
        let mut stream = tokio_stream::iter(self.data.clone());
        while let Some(user) = stream.next().await {
            let result = user_service.find_by_email(&user.email).await?;
            if let Some(user) = result {
                warn!("User: {} already existed", user.email);
                continue;
            }

            let user = user_service
                .create_with_role(user.email, user.username, user.password, user.role)
                .await?;
            info!("User: {} created", user.email)
        }

        Ok(())
    }

    fn path() -> &'static str {
        "./seed/users"
    }
}
