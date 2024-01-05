use async_graphql::{Context, Object, Result};

use crate::api::dto::{ThreadVersionBy, ThreadVersionBySemver};
use crate::api::models::ThreadVersion;
use crate::api::services::ThreadVersionServiceDyn;
use crate::error::AppError;

#[derive(Default)]
pub struct ThreadVersionQuery;

#[Object]
impl ThreadVersionQuery {
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
