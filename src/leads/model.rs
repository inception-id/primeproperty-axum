use super::controller::CreateLeadPayload;
use diesel::{QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

use crate::{db::DbPool, schema::leads, traits::Crud};

#[derive(Serialize, Queryable)]
pub(super) struct Lead {
    id: i32,
    user_id: uuid::Uuid,
    property_id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    name: String,
    phone_number: String,
    email: Option<String>,
    is_deleted: bool,
}

impl Crud for Lead {
    type Output = Self;
    type SchemaTable = leads::table;
    type CreatePayload = CreateLeadPayload;

    fn create(
        pool: &DbPool,
        #[allow(unused)] uuid: &uuid::Uuid,
        payload: &Self::CreatePayload,
    ) -> QueryResult<Self::Output> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::insert_into(leads::table)
            .values(payload)
            .get_result(conn)
    }
}
