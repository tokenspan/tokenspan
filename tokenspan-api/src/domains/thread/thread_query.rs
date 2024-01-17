use crate::domains::models::ParsedToken;
use async_graphql::connection::Connection;
use async_graphql::{Context, ErrorExtensions, Object, Result};
use uuid::Uuid;

use crate::domains::services::ThreadServiceDyn;
use crate::domains::thread::dto::ThreadArgs;
use crate::domains::thread::thread_model::Thread;
use crate::errors::AppError;
use dojo_orm::pagination::{AdditionalFields, Cursor};

#[derive(Default)]
pub struct ThreadQuery;

#[Object]
impl ThreadQuery {
    pub async fn threads<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(default)] args: ThreadArgs,
    ) -> Result<Connection<Cursor, Thread, AdditionalFields>> {
        let thread_service = ctx
            .data::<ThreadServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_thread = thread_service.paginate(args).await?;

        Ok(paginated_thread.into())
    }

    pub async fn threads_by_owner<'a>(
        &self,
        ctx: &Context<'a>,
        args: ThreadArgs,
    ) -> Result<Connection<Cursor, Thread, AdditionalFields>> {
        let thread_service = ctx
            .data::<ThreadServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        let paginated_thread = thread_service
            .find_by_owner(&parsed_token.user_id, args)
            .await?;

        Ok(paginated_thread.into())
    }

    pub async fn thread<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Option<Thread>> {
        let thread_service = ctx
            .data::<ThreadServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let thread = thread_service.find_by_id(&id).await?;

        Ok(thread)
    }

    pub async fn count_threads<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Option<Thread>> {
        let thread_service = ctx
            .data::<ThreadServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let thread = thread_service.find_by_id(&id).await?;

        Ok(thread)
    }
}
