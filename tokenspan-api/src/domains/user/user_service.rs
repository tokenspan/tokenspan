use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::pagination::Pagination;
use dojo_orm::prelude::*;
use dojo_orm::Database;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::domains::dto::{UserArgs, UserUpdateInput};
use crate::domains::models::{User, UserRole};

#[async_trait::async_trait]
pub trait UserServiceExt {
    async fn paginate(&self, args: UserArgs) -> Result<Pagination<User>>;
    async fn create(&self, email: String, username: String, password: String) -> Result<User>;
    async fn create_with_role(
        &self,
        email: String,
        username: String,
        password: String,
        role: UserRole,
    ) -> Result<User>;
    async fn update_by_id(&self, id: &Uuid, input: UserUpdateInput) -> Result<User>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<User>>;
    async fn find_by_email(&self, email: &String) -> Result<Option<User>>;
}

pub type UserServiceDyn = Arc<dyn UserServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct UserService {
    db: Database,
}

#[async_trait::async_trait]
impl UserServiceExt for UserService {
    async fn paginate(&self, args: UserArgs) -> Result<Pagination<User>> {
        let mut predicates = vec![];
        if let Some(r#where) = &args.r#where {
            if let Some(username) = &r#where.username {
                predicates.push(text_search("username", "english", username));
            }

            if let Some(email) = &r#where.email {
                predicates.push(text_search("email", "simple", email));
            }
        }

        self.db
            .bind::<User>()
            .where_by(and(&predicates))
            .cursor(args.first, args.after, args.last, args.before)
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
        let (hash_password, salt) = User::hash_password(password.as_bytes())?;

        let input = User {
            id: Uuid::new_v4(),
            email,
            username,
            password: hash_password,
            salt,
            role,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db.insert(&input).exec().await
    }

    async fn update_by_id(&self, id: &Uuid, input: UserUpdateInput) -> Result<User> {
        self.db
            .update(&input)
            .where_by(equals("id", id))
            .exec()
            .await
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>> {
        self.db
            .bind::<User>()
            .where_by(equals("id", id))
            .first()
            .await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<User>> {
        self.db
            .bind::<User>()
            .where_by(in_list("id", &ids))
            .all()
            .await
    }

    async fn find_by_email(&self, email: &String) -> Result<Option<User>> {
        self.db
            .bind::<User>()
            .where_by(equals("email", email))
            .first()
            .await
    }
}

impl From<UserService> for UserServiceDyn {
    fn from(value: UserService) -> Self {
        Arc::new(value) as Self
    }
}
