use async_graphql::InputObject;
use sea_orm::ActiveValue::Set;

use crate::api::models::TaskId;

#[derive(InputObject)]
pub struct TaskVersionCreateInput {
    pub task_id: TaskId,
    pub semver: String,
    pub version: u32,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
}

#[derive(InputObject)]
pub struct TaskVersionUpdateInput {
    pub semver: Option<String>,
    pub version: Option<u32>,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
}

impl TaskVersionUpdateInput {
    pub fn merge(&self, input: &mut entity::task_version::ActiveModel) {
        if let Some(ref semver) = self.semver {
            input.semver = Set(semver.clone());
        }

        if let Some(version) = self.version {
            input.version = Set(version as i32);
        }

        if let Some(ref release_note) = self.release_note {
            input.release_note = Set(Some(release_note.clone()));
        }

        if let Some(ref description) = self.description {
            input.description = Set(Some(description.clone()));
        }

        if let Some(ref document) = self.document {
            input.document = Set(Some(document.clone()));
        }
    }
}
