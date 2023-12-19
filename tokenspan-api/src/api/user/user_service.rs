use std::num::NonZeroU32;
use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::dto::{UserArgs, UserUpdateInput};
use crate::api::models::UserRole;
use crate::api::user::user_error::UserError;
use crate::api::user::user_model::User;
use crate::api::user::user_repository::UserRepository;
use crate::repository::RepositoryExt;
use crate::set_optional;

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
    async fn update_by_id(&self, id: Uuid, input: UserUpdateInput) -> Result<User>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<User>>;
    async fn find_by_email(&self, email: String) -> Result<Option<User>>;
    fn verify_password(&self, password: &str, salt: &str, hash_password: &str) -> Result<()>;
}

pub type UserServiceDyn = Arc<dyn UserServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    const ITERATIONS: u32 = 100_000;

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
        self.user_repo
            .paginate(args.take.unwrap_or(10), args.after, args.before)
            .await
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

        let user = self
            .user_repo
            .create(|s| {
                s.push_bind(Uuid::new_v4());
                s.push_bind(email);
                s.push_bind(username);
                s.push_bind(hash_password);
                s.push_bind(salt);
                s.push_bind(role);
                s.push_bind(Utc::now().naive_utc());
                s.push_bind(Utc::now().naive_utc());
            })
            .await?;

        Ok(user)
    }

    async fn update_by_id(&self, id: Uuid, input: UserUpdateInput) -> Result<User> {
        let user = self
            .user_repo
            .update_by_id(id, |sep| {
                set_optional!(sep, "email", input.email);
                set_optional!(sep, "username", input.username);
            })
            .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        self.user_repo.find_by_id(id).await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<User>> {
        self.user_repo.find_by_ids(ids).await
    }

    async fn find_by_email(&self, email: String) -> Result<Option<User>> {
        self.user_repo.find_by("email", email).await
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
