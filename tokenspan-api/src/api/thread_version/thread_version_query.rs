use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};
use dojo_orm::pagination::{AdditionalFields, Cursor};

use crate::api::dto::{ThreadVersionArgs, ThreadVersionBy, ThreadVersionBySemver};
use crate::api::models::ThreadVersion;
use crate::api::services::{ProviderServiceDyn, ThreadVersionServiceDyn};
use crate::error::AppError;

#[derive(Default)]
pub struct ThreadVersionQuery;

#[Object]
impl ThreadVersionQuery {
    pub async fn thread_versions<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(default)] args: ThreadVersionArgs,
    ) -> Result<Connection<Cursor, ThreadVersion, AdditionalFields>> {
        let thread_version_service = ctx
            .data::<ThreadVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_thread_version = thread_version_service.paginate(args).await?;

        Ok(paginated_thread_version.into())
    }
    pub async fn thread_version<'a>(
        &self,
        ctx: &Context<'a>,
        by: ThreadVersionBy,
    ) -> Result<Option<ThreadVersion>> {
        let thread_version_service = ctx
            .data::<ThreadVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let thread_version = match by {
            ThreadVersionBy::Id(id) => thread_version_service.find_by_id(id).await,
            ThreadVersionBy::Semver(ThreadVersionBySemver { thread_id, semver })
                if semver == "latest".to_string() =>
            {
                thread_version_service.find_latest(thread_id).await
            }
            ThreadVersionBy::Semver(version) => {
                thread_version_service
                    .find_by_semver(version.thread_id, version.semver)
                    .await
            }
            ThreadVersionBy::Latest(latest) => {
                thread_version_service.find_latest(latest.thread_id).await
            }
        }?;

        Ok(thread_version)
    }
}
