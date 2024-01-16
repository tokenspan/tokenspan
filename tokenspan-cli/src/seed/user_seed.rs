use async_trait::async_trait;
use dojo_orm::prelude::equals;
use dojo_orm::Database;
use tracing::{info, warn};

use tokenspan_api::api::models::User;

use crate::seed::Seed;

pub struct UserSeed<'a> {
    pub db: &'a Database,
}

impl<'a> UserSeed<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<'a> Seed for UserSeed<'a> {
    async fn save(&self) -> anyhow::Result<()> {
        let items = Self::load::<User>().await?;

        for item in items {
            let user = self
                .db
                .bind::<User>()
                .where_by(equals("id", &item.id))
                .first()
                .await?;
            if user.is_some() {
                warn!("User {} already exists", item.id);
                continue;
            }

            let mut user = item;
            let (hash_password, salt) = User::hash_password(user.password.as_bytes())?;
            user.password = hash_password;
            user.salt = salt;

            self.db.insert(&user).await?;
            info!("User {} created", user.id);
        }

        Ok(())
    }

    fn path() -> &'static str {
        "./seed/users"
    }
}
