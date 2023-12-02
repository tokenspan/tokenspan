use crate::api::models::ProviderId;
use crate::repository::Repository;
use bson::doc;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderEntity {
    pub id: ObjectId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderCreateEntity {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderUpdateEntity {
    pub name: Option<String>,
}

impl Repository<ProviderEntity> {
    pub async fn create(&self, doc: ProviderCreateEntity) -> Result<ProviderEntity> {
        let doc = ProviderEntity {
            id: ObjectId::new(),
            name: doc.name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.collection.insert_one(doc, None).await?;
        let id = result
            .inserted_id
            .as_object_id()
            .map(ProviderId::from)
            .ok_or(Error::custom("invalid id"))?;

        self.find_by_id(id)
            .await?
            .ok_or(Error::custom("user not found"))
    }

    pub async fn update_by_id(
        &self,
        id: ProviderId,
        doc: ProviderUpdateEntity,
    ) -> Result<Option<ProviderEntity>> {
        let filter = doc! {
            "_id": ObjectId::from(id),
        };
        let update = doc! {
            "$set": {
                "name": doc.name,
                "updated_at": Utc::now(),
            }
        };

        self.collection
            .find_one_and_update(filter, update, None)
            .await
    }
}
