use super::controller::{CreateLeadPayload, FindLeadQueryParam};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

use crate::{db::DbPool, schema::leads, traits::Crud};

#[derive(Serialize, Queryable)]
pub struct Lead {
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

impl Lead {
    pub fn delete_by_property_id(pool: &DbPool, property_id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::update(leads::table)
            .filter(leads::property_id.eq(property_id))
            .set(leads::is_deleted.eq(true))
            .get_result(conn)
    }
}

impl Crud for Lead {
    type Output = Self;
    type SchemaTable = leads::table;
    type CreatePayload = CreateLeadPayload;
    type FindManyOutput = Self;
    type FindManyParam = FindLeadQueryParam;

    fn create(
        pool: &DbPool,
        #[allow(unused_variables)] uuid: &uuid::Uuid,
        payload: &Self::CreatePayload,
    ) -> QueryResult<Self::Output> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::insert_into(leads::table)
            .values(payload)
            .get_result(conn)
    }

    fn find_many(
        pool: &DbPool,
        #[allow(unused_variables)] user_id: &Option<uuid::Uuid>,
        #[allow(unused_variables)] role: &Option<crate::agents::AgentRole>,
        #[allow(unused_variables)] query_params: &Self::FindManyParam,
    ) -> QueryResult<Vec<Self::FindManyOutput>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        leads::table
            .order_by(leads::created_at.desc())
            .get_results(conn)
    }

    fn count_find_many_rows(
        pool: &DbPool,
        #[allow(unused_variables)] user_id: &Option<uuid::Uuid>,
        #[allow(unused_variables)] role: &Option<crate::agents::AgentRole>,
        #[allow(unused_variables)] query_params: &Self::FindManyParam,
    ) -> QueryResult<i64> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        leads::table.count().get_result(conn)
    }
}
