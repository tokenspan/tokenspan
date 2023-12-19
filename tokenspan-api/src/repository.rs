use async_trait::async_trait;
use chrono::NaiveDateTime;
use futures_util::TryFutureExt;
use sqlx::postgres::PgRow;
use sqlx::query_builder::Separated;
use sqlx::{Encode, FromRow, Pool, Postgres, QueryBuilder, Row, Type};
use uuid::Uuid;

use tokenspan_extra::pagination::{Cursor, CursorExt, Pagination};
use tokenspan_extra::FieldNamesExt;

#[async_trait]
pub trait RepositoryExt<T>
where
    T: CursorExt<Cursor> + for<'a> FromRow<'a, PgRow> + Send + Sync + Unpin + FieldNamesExt,
{
    fn pool(&self) -> &Pool<Postgres>;

    fn table() -> &'static str;

    fn columns(&self) -> &'static [&'static str] {
        T::FIELDS
    }

    async fn paginate(
        &self,
        limit: u64,
        after: Option<Cursor>,
        before: Option<Cursor>,
    ) -> anyhow::Result<Pagination<Cursor, T>> {
        let new_limit = limit
            + if after.is_some() || before.is_some() {
                2
            } else {
                1
            };

        let mut sql = QueryBuilder::new("SELECT ");
        sql.push(self.columns().join(", "));
        sql.push(" FROM ");
        sql.push(Self::table());

        if let Some(after) = after {
            let after: NaiveDateTime = after.try_into()?;
            sql.push(" WHERE created_at <= ");
            sql.push_bind(after);
        }

        if let Some(before) = before {
            let before: NaiveDateTime = before.try_into()?;
            sql.push(" WHERE created_at >= ");
            sql.push_bind(before);
        }

        sql.push(" ORDER BY created_at DESC LIMIT ");
        sql.push_bind(new_limit as i64);

        let query = sql.build_query_as::<T>();
        let fetch_records_fut = query.fetch_all(self.pool()).map_err(|e| anyhow::anyhow!(e));

        let (records, count) = tokio::try_join!(fetch_records_fut, self.count())?;

        Ok(Pagination::new(records, before, after, limit, count as u64))
    }

    async fn count(&self) -> anyhow::Result<i64> {
        let mut sql = QueryBuilder::new("SELECT COUNT(*) FROM ");
        sql.push(Self::table());

        let query = sql.build();
        let count = query
            .fetch_one(self.pool())
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(count.get(0))
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<T>> {
        let mut sql = QueryBuilder::new("SELECT ");
        sql.push(self.columns().join(", "));
        sql.push(" FROM ");
        sql.push(Self::table());
        sql.push(" WHERE id = ");
        sql.push_bind(id);

        let query = sql.build_query_as::<T>();
        let record = query
            .fetch_optional(self.pool())
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(record)
    }

    async fn find_by<U>(&self, field: &str, value: U) -> anyhow::Result<Option<T>>
    where
        U: Type<Postgres> + for<'a> Encode<'a, Postgres> + Send,
    {
        let mut sql = QueryBuilder::new("SELECT ");
        sql.push(self.columns().join(", "));
        sql.push(" FROM ");
        sql.push(Self::table());
        sql.push(" WHERE ");
        sql.push(field);
        sql.push(" = ");
        sql.push_bind(value);

        let query = sql.build_query_as::<T>();
        let user = query
            .fetch_optional(self.pool())
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(user)
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> anyhow::Result<Vec<T>> {
        let mut sql = QueryBuilder::new("SELECT ");
        sql.push(self.columns().join(", "));
        sql.push(" FROM ");
        sql.push(Self::table());
        sql.push(" WHERE id IN ");
        sql.push_bind(ids);

        let query = sql.build_query_as::<T>();
        let records = query
            .fetch_all(self.pool())
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(records)
    }

    async fn delete_by_id(&self, id: Uuid) -> anyhow::Result<T> {
        let mut sql = QueryBuilder::new("DELETE FROM ");
        sql.push(Self::table());
        sql.push(" WHERE id = ");
        sql.push_bind(id);
        sql.push(" RETURNING ");
        sql.push(self.columns().join(", "));

        let query = sql.build_query_as::<T>();
        let record = query
            .fetch_one(self.pool())
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(record)
    }
    async fn create<F>(&self, set: F) -> anyhow::Result<T>
    where
        F: FnOnce(&mut Separated<Postgres, &str>) -> () + Send,
    {
        let mut sql = QueryBuilder::new("INSERT INTO ");
        sql.push(Self::table());
        sql.push(" (");
        sql.push(self.columns().join(", "));
        sql.push(") VALUES (");

        let mut separated = sql.separated(", ");
        set(&mut separated);
        separated.push_unseparated(") RETURNING ");

        sql.push(self.columns().join(", "));

        let query = sql.build_query_as::<T>();
        let record = query
            .fetch_one(self.pool())
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(record)
    }

    async fn update_by_id<F>(&self, id: Uuid, set: F) -> anyhow::Result<T>
    where
        F: FnOnce(&mut Separated<Postgres, &str>) -> () + Send,
    {
        let mut sql = QueryBuilder::new("UPDATE ");
        sql.push(Self::table());
        sql.push(" SET ");

        let mut separated = sql.separated(", ");
        set(&mut separated);
        separated.push_unseparated(" WHERE id = ");
        separated.push_bind(id);
        separated.push_unseparated(" RETURNING ");

        sql.push(self.columns().join(", "));

        let query = sql.build_query_as::<T>();
        let record = query
            .fetch_one(self.pool())
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(record)
    }
}

#[macro_export]
macro_rules! set_optional {
    ($sep:ident, $field:literal, $value:expr) => {
        if let Some(value) = $value {
            $sep.push($field);
            $sep.push(" = ");
            $sep.push_bind(value);
        }
    };
}

#[macro_export]
macro_rules! set {
    ($sep:ident, $field:literal, $value:expr) => {
        $sep.push($field);
        $sep.push(" = ");
        $sep.push_bind(value);
    };
}
