use diesel::{Insertable, QueryResult, Table};

use crate::db::DbPool;

pub trait Crud {
    type Output;
    type SchemaTable: Table;
    type CreatePayload: Insertable<Self::SchemaTable>;

    fn schema_table() -> Self::SchemaTable;

    fn create(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &Self::CreatePayload,
    ) -> QueryResult<Self::Output>;
}
