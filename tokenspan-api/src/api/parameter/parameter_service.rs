use std::sync::Arc;

use async_graphql::Result;
use prisma_client_rust::Direction;

use tokenspan_utils::pagination::{Cursor, Pagination};

use crate::api::models::ParameterId;
use crate::api::parameter::dto::{CreateParameterInput, ParameterArgs, UpdateParameterInput};
use crate::api::parameter::parameter_error::ParameterError;
use crate::api::parameter::parameter_model::Parameter;
use crate::prisma::{model, parameter, PrismaClient};

#[async_trait::async_trait]
pub trait ParameterServiceExt {
    async fn get_parameters(&self, args: ParameterArgs) -> Result<Pagination<Cursor, Parameter>>;
    async fn get_parameter_by_id(&self, id: ParameterId) -> Result<Option<Parameter>>;
    async fn get_parameters_by_ids(&self, ids: Vec<ParameterId>) -> Result<Vec<Parameter>>;
    async fn count_parameters(&self) -> Result<i64>;
    async fn create_parameter(&self, input: CreateParameterInput) -> Result<Parameter>;
    async fn update_parameter(
        &self,
        id: ParameterId,
        input: UpdateParameterInput,
    ) -> Result<Parameter>;
    async fn delete_parameter(&self, id: ParameterId) -> Result<Parameter>;
}

pub type ParameterServiceDyn = Arc<dyn ParameterServiceExt + Send + Sync>;

pub struct ParameterService {
    prisma: Arc<PrismaClient>,
}

impl ParameterService {
    pub fn new(prisma: Arc<PrismaClient>) -> Self {
        Self { prisma }
    }
}

#[async_trait::async_trait]
impl ParameterServiceExt for ParameterService {
    async fn get_parameters(&self, args: ParameterArgs) -> Result<Pagination<Cursor, Parameter>> {
        let take = args.take.unwrap_or(1);

        let builder = self
            .prisma
            .parameter()
            .find_many(vec![])
            .take(take + 1)
            .order_by(parameter::id::order(Direction::Desc));

        let builder = match (&args.before, &args.after) {
            (Some(cursor), None) => builder
                .take((take + 2) * -1)
                .cursor(parameter::id::equals(cursor.id.clone())),
            (None, Some(cursor)) => builder
                .take(take + 2)
                .cursor(parameter::id::equals(cursor.id.clone())),
            _ => builder,
        };

        let items = builder
            .exec()
            .await
            .map_err(|_| ParameterError::UnableToGetParameters)?
            .into_iter()
            .map(|data| data.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take))
    }

    async fn get_parameter_by_id(&self, id: ParameterId) -> Result<Option<Parameter>> {
        let parameter = self
            .prisma
            .parameter()
            .find_unique(parameter::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| ParameterError::UnableToGetParameter)?
            .map(|parameter| parameter.into());

        Ok(parameter)
    }

    async fn get_parameters_by_ids(&self, ids: Vec<ParameterId>) -> Result<Vec<Parameter>> {
        let ids = ids
            .into_iter()
            .map(|id| parameter::id::equals(id.into()))
            .collect();
        let parameters = self
            .prisma
            .parameter()
            .find_many(ids)
            .exec()
            .await
            .map_err(|_| ParameterError::UnableToGetParameters)?
            .into_iter()
            .map(|parameter| parameter.into())
            .collect();

        Ok(parameters)
    }

    async fn count_parameters(&self) -> Result<i64> {
        let count = self
            .prisma
            .parameter()
            .count(vec![])
            .exec()
            .await
            .map_err(|_| ParameterError::UnableToCountParameters)?;

        Ok(count)
    }

    async fn create_parameter(&self, input: CreateParameterInput) -> Result<Parameter> {
        let created_parameter = self
            .prisma
            .parameter()
            .create(
                model::id::equals(input.model_id.into()),
                input.name,
                input.temperature,
                input.max_tokens,
                input.top_p,
                input.frequency_penalty,
                input.presence_penalty,
                vec![],
            )
            .exec()
            .await
            .map_err(|_| ParameterError::UnableToCreateParameter)?;

        Ok(created_parameter.into())
    }

    async fn update_parameter(
        &self,
        id: ParameterId,
        input: UpdateParameterInput,
    ) -> Result<Parameter> {
        let updated_parameter = self
            .prisma
            .parameter()
            .update(parameter::id::equals(id.into()), input.into())
            .exec()
            .await
            .map_err(|_| ParameterError::UnableToUpdateParameter)?;

        Ok(updated_parameter.into())
    }

    async fn delete_parameter(&self, id: ParameterId) -> Result<Parameter> {
        let deleted_parameter = self
            .prisma
            .parameter()
            .delete(parameter::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| ParameterError::UnableToDeleteParameter)?;

        Ok(deleted_parameter.into())
    }
}

impl From<ParameterService> for ParameterServiceDyn {
    fn from(value: ParameterService) -> Self {
        Arc::new(value) as Self
    }
}
