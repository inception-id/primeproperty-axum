use crate::db::DbPool;
use diesel::{Insertable, QueryResult, Table};
use serde::de::DeserializeOwned;

pub const PAGE_SIZE: i64 = 15;

pub trait Crud {
    type Output;
    type SchemaTable: Table;
    type CreatePayload: Insertable<Self::SchemaTable> + DeserializeOwned;

    fn create(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &Self::CreatePayload,
    ) -> QueryResult<Self::Output>;
}
