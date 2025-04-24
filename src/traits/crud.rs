use crate::db::DbPool;
use diesel::{Insertable, QueryResult, Table};
use serde::de::DeserializeOwned;

pub trait Crud {
    type Output;
    type SchemaTable: Table;
    type CreatePayload: Insertable<Self::SchemaTable> + DeserializeOwned;
    type FindQueries: DeserializeOwned;

    const PAGE_SIZE: i64 = 15;

    fn create(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &Self::CreatePayload,
    ) -> QueryResult<Self::Output>;

    fn find_many_by_user_id(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        queries: &Self::FindQueries,
    ) -> QueryResult<Vec<Self::Output>>;

    fn count_find_many_by_user_id_total(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        queries: &Self::FindQueries,
    ) -> QueryResult<i64>;
}
