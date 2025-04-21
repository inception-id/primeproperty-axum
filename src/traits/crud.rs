use crate::db::DbPool;
use diesel::{Insertable, QueryResult, Table};
use serde::de::DeserializeOwned;

pub const PAGE_SIZE: i64 = 10;

pub trait Crud {
    type Output;
    type SchemaTable: Table;
    type CreatePayload: Insertable<Self::SchemaTable> + DeserializeOwned;
    type FindQueries: DeserializeOwned;

    fn schema_table() -> Self::SchemaTable;

    fn create(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &Self::CreatePayload,
    ) -> QueryResult<Self::Output>;

    fn find(pool: &DbPool, queries: &Self::FindQueries) -> QueryResult<Vec<Self::Output>>;
}
