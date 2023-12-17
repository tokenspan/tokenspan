use std::num::NonZeroU32;
use std::sync::Arc;

use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use uuid::Uuid;

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::dto::UserArgs;
use crate::api::models::UserRole;
use crate::api::user::user_error::UserError;
use crate::api::user::user_model::User;

#[async_trait::async_trait]
pub trait UserServiceExt {
    async fn paginate(&self, args: UserArgs) -> Result<Pagination<Cursor, User>>;
    async fn create(&self, email: String, username: String, password: String) -> Result<User>;
    async fn create_with_role(
        &self,
        email: String,
        username: String,
        password: String,
        role: UserRole,
    ) -> Result<User>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<User>>;
    async fn find_by_email(&self, email: String) -> Result<Option<User>>;
    fn verify_password(&self, password: &str, salt: &str, hash_password: &str) -> Result<()>;
}

pub type UserServiceDyn = Arc<dyn UserServiceExt + Send + Sync>;

pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    const ITERATIONS: u32 = 100_000;

    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn derive_password(&self, password: String) -> Result<([u8; 64], [u8; 64])> {
        let iterations = NonZeroU32::new(Self::ITERATIONS).ok_or(UserError::InvalidIterations)?;
        let rng = rand::SystemRandom::new();

        let mut salt = [0u8; Self::CREDENTIAL_LEN];
        rng.fill(&mut salt).unwrap();

        let mut pbkdf2_hash = [0u8; Self::CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            iterations,
            &salt,
            password.as_bytes(),
            &mut pbkdf2_hash,
        );

        Ok((pbkdf2_hash, salt))
    }
}

#[async_trait::async_trait]
impl UserServiceExt for UserService {
    async fn paginate(&self, args: UserArgs) -> Result<Pagination<Cursor, User>> {
        let take = args.take.unwrap_or(10);
        let limit = take
            + if args.after.is_some() || args.before.is_some() {
                2
            } else {
                1
            };
        let mut select = entity::user::Entity::find()
            .limit(Some(limit))
            .order_by_desc(entity::user::Column::CreatedAt);

        if let Some(after) = args.after.clone() {
            let after: NaiveDateTime = after.try_into()?;
            select = select.filter(entity::user::Column::CreatedAt.lte(after));
        }

        if let Some(before) = args.before.clone() {
            let before: NaiveDateTime = before.try_into()?;
            select = select.filter(entity::user::Column::CreatedAt.gte(before));
        }

        let count_fut = entity::user::Entity::find().count(&self.db);
        let select_fut = select.all(&self.db);

        let (count, items) = tokio::join!(count_fut, select_fut);

        let count = count.map_err(|e| UserError::Unknown(anyhow::anyhow!(e)))?;
        let items = items
            .map_err(|e| UserError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|execution| execution.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take, count))
    }

    async fn create(&self, email: String, username: String, password: String) -> Result<User> {
        self.create_with_role(email, username, password, UserRole::User)
            .await
    }

    async fn create_with_role(
        &self,
        email: String,
        username: String,
        password: String,
        role: UserRole,
    ) -> Result<User> {
        let (hash_password, salt) = self.derive_password(password.clone()).unwrap();
        let hash_password = HEXUPPER.encode(&hash_password);
        let salt = HEXUPPER.encode(&salt);

        let created_user = entity::user::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(email),
            username: Set(username),
            password: Set(hash_password),
            salt: Set(salt),
            role: Set(role.into()),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
        .insert(&self.db)
        .await
        .map_err(|e| UserError::Unknown(anyhow::anyhow!(e)))?;

        Ok(created_user.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = entity::user::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| UserError::Unknown(anyhow::anyhow!(e)))?
            .map(|user| user.into());

        Ok(user)
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<User>> {
        let users = entity::user::Entity::find()
            .filter(entity::user::Column::Id.is_in(ids.to_vec()))
            .all(&self.db)
            .await
            .map_err(|e| UserError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|user| user.into())
            .collect();

        Ok(users)
    }

    async fn find_by_email(&self, email: String) -> Result<Option<User>> {
        let user = entity::user::Entity::find()
            .filter(entity::user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| UserError::Unknown(anyhow::anyhow!(e)))?
            .map(|user| user.into());

        Ok(user)
    }

    fn verify_password(&self, password: &str, salt: &str, hash_password: &str) -> Result<()> {
        let iterations = NonZeroU32::new(Self::ITERATIONS).ok_or(UserError::InvalidIterations)?;
        let hash_password = HEXUPPER.decode(hash_password.as_bytes()).map_err(|e| {
            UserError::Unknown(anyhow::anyhow!("failed to decode hash password: {}", e))
        })?;
        let salt = HEXUPPER.decode(salt.as_bytes()).unwrap();
        pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            iterations,
            salt.as_slice(),
            password.as_bytes(),
            hash_password.as_slice(),
        )?;

        Ok(())
    }
}

impl From<UserService> for UserServiceDyn {
    fn from(value: UserService) -> Self {
        Arc::new(value) as Self
    }
}
