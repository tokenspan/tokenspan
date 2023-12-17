use async_graphql::InputObject;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(InputObject, TypedBuilder)]
pub struct TaskVersionCreateInput {
    pub task_id: Uuid,
    #[builder(default = "0.0.0".to_string())]
    pub semver: String,
    #[builder(default = 0)]
    pub version: u32,
    #[builder(default)]
    pub release_note: Option<String>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
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
    pub fn copy(&self, model: &mut entity::task_version::ActiveModel) {
        if let Some(ref semver) = self.semver {
            model.semver = Set(semver.clone());
        }

        if let Some(version) = self.version {
            model.version = Set(version as i32);
        }

        if let Some(ref release_note) = self.release_note {
            model.release_note = Set(Some(release_note.clone()));
        }

        if let Some(ref description) = self.description {
            model.description = Set(Some(description.clone()));
        }

        if let Some(ref document) = self.document {
            model.document = Set(Some(document.clone()));
        }

        model.updated_at = Set(Utc::now().naive_utc());
    }
}
