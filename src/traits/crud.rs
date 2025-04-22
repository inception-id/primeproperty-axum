use crate::db::DbPool;
use diesel::{Insertable, QueryResult, Table};
use serde::de::DeserializeOwned;

pub trait Crud {
    type Output;
    type SchemaTable: Table;
    type CreatePayload: Insertable<Self::SchemaTable> + DeserializeOwned;
    type FindQueries: DeserializeOwned;

    const PAGE_SIZE: i64 = 10;

    fn schema_table() -> Self::SchemaTable;

    fn create(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &Self::CreatePayload,
    ) -> QueryResult<Self::Output>;

    // TODO:  Count page total
    fn find(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        queries: &Self::FindQueries,
    ) -> QueryResult<Vec<Self::Output>>;

    fn count_find_total(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        queries: &Self::FindQueries,
    ) -> QueryResult<i64>;
}
