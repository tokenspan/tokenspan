use std::num::NonZeroU32;
use std::sync::Arc;

use async_graphql::Result;
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};

use crate::api::models::UserId;
use crate::api::user::user_error::UserError;
use crate::api::user::user_model::User;
use crate::prisma::{user, PrismaClient};

#[async_trait::async_trait]
pub trait UserServiceExt {
    async fn create_user(&self, email: String, username: String, password: String) -> Result<User>;
    async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>>;
    async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>>;
    async fn get_user_by_username(&self, email: String) -> Result<Option<User>>;
    fn verify_password(&self, password: &str, salt: &str, hash_password: &str) -> Result<()>;
}

pub type UserServiceDyn = Arc<dyn UserServiceExt + Send + Sync>;

pub struct UserService {
    prisma: Arc<PrismaClient>,
}

impl UserService {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    const ITERATIONS: u32 = 100_000;

    pub fn new(prisma: Arc<PrismaClient>) -> Self {
        Self { prisma }
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
    async fn create_user(&self, email: String, username: String, password: String) -> Result<User> {
        let (hash_password, salt) = self.derive_password(password.clone()).unwrap();
        let hash_password = HEXUPPER.encode(&hash_password);
        let salt = HEXUPPER.encode(&salt);

        let created_user = self
            .prisma
            .user()
            .create(email, hash_password, salt, username, vec![])
            .exec()
            .await
            .map_err(|_| UserError::UserNotFound(None))?;

        Ok(created_user.into())
    }

    async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>> {
        let user = self
            .prisma
            .user()
            .find_unique(user::id::equals(id.0))
            .exec()
            .await
            .map_err(|_| UserError::UserNotFound(None))?
            .map(|user| user.into());

        Ok(user)
    }

    async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>> {
        let ids = ids.into_iter().map(|id| user::id::equals(id.0)).collect();
        let users = self
            .prisma
            .user()
            .find_many(ids)
            .exec()
            .await
            .map_err(|_| UserError::UserNotFound(None))?
            .into_iter()
            .map(|user| user.into())
            .collect();

        Ok(users)
    }

    async fn get_user_by_username(&self, email: String) -> Result<Option<User>> {
        let user = self
            .prisma
            .user()
            .find_unique(user::email::equals(email))
            .exec()
            .await
            .map_err(|_| UserError::UserNotFound(None))?
            .map(|user| user.into());

        Ok(user)
    }

    fn verify_password(&self, password: &str, salt: &str, hash_password: &str) -> Result<()> {
        let iterations = NonZeroU32::new(Self::ITERATIONS).ok_or(UserError::InvalidIterations)?;
        let hash_password = HEXUPPER.decode(hash_password.as_bytes()).unwrap();
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
