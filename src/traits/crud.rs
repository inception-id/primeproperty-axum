use crate::{agents::AgentRole, db::DbPool};
use diesel::{Insertable, QueryResult, Table};
use serde::de::DeserializeOwned;

pub trait Crud {
    type Output;
    type SchemaTable: Table;
    type CreatePayload: Insertable<Self::SchemaTable> + DeserializeOwned;
    type FindManyOutput;
    type FindManyParam: DeserializeOwned;

    fn create(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &Self::CreatePayload,
    ) -> QueryResult<Self::Output>;

    fn find_many(
        pool: &DbPool,
        user_id: &Option<uuid::Uuid>,
        role: &Option<AgentRole>,
        query_params: &Self::FindManyParam,
    ) -> QueryResult<Vec<Self::FindManyOutput>>;

    fn count_find_many_rows(
        pool: &DbPool,
        user_id: &Option<uuid::Uuid>,
        role: &Option<AgentRole>,
        query_params: &Self::FindManyParam,
    ) -> QueryResult<i64>;
}
