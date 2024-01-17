use crate::domains::user::user_error::UserError;
use async_graphql::{Enum, SimpleObject};
use chrono::NaiveDateTime;
use data_encoding::HEXUPPER;
use dojo_macros::{Model, Type};
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use serde::Deserialize;
use std::num::NonZeroU32;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(SimpleObject, Clone, Debug, Model, Deserialize)]
#[dojo(name = "users", sort_keys = ["created_at", "id"])]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[graphql(skip)]
    pub password: String,
    #[graphql(skip)]
    pub salt: String,
    pub role: UserRole,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    const ITERATIONS: u32 = 100_000;

    pub fn hash_password(password: &[u8]) -> anyhow::Result<(String, String)> {
        let iterations = NonZeroU32::new(Self::ITERATIONS).ok_or(UserError::InvalidIterations)?;
        let rng = rand::SystemRandom::new();

        let mut salt = [0u8; Self::CREDENTIAL_LEN];
        rng.fill(&mut salt).unwrap();

        let mut pbkdf2_hash = [0u8; Self::CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            iterations,
            &salt,
            password,
            &mut pbkdf2_hash,
        );

        let hash_password = HEXUPPER.encode(&pbkdf2_hash);
        let salt = HEXUPPER.encode(&salt);

        Ok((hash_password, salt))
    }

    pub fn verify_password(&self, password: &str) -> anyhow::Result<bool> {
        let iterations = NonZeroU32::new(Self::ITERATIONS).ok_or(UserError::InvalidIterations)?;
        let hash_password = HEXUPPER
            .decode(self.password.as_bytes())
            .map_err(|e| anyhow::anyhow!("failed to decode hash password: {}", e))?;
        let salt = HEXUPPER
            .decode(self.salt.as_bytes())
            .map_err(|e| anyhow::anyhow!("failed to decode salt: {}", e))?;
        pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            iterations,
            salt.as_slice(),
            password.as_bytes(),
            hash_password.as_slice(),
        )?;

        Ok(true)
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, Display, EnumString, Deserialize, Type)]
#[dojo(name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    #[strum(serialize = "admin")]
    #[serde(rename = "admin")]
    Admin,
    #[strum(serialize = "user")]
    #[serde(rename = "user")]
    User,
}
