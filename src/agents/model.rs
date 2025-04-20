use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

use super::agent_role::AgentRole;
use super::controller::CreateAgentPayload;
use crate::db::DbPool;
use crate::schema::agents;
use crate::traits::Crud;

#[derive(Debug, Serialize, Queryable)]
pub struct Agent {
    id: uuid::Uuid,
    supertokens_user_id: Option<String>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    fullname: String,
    email: String,
    phone_number: String,
    profile_picture_url: Option<String>,
    pub role: AgentRole,
}

impl Agent {
    pub(super) fn find_by_supertokens_user_id(
        pool: &DbPool,
        supertokens_user_id: &str,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        agents::table
            .filter(agents::supertokens_user_id.eq(supertokens_user_id))
            .get_result(conn)
    }

    pub fn find_by_id(pool: &DbPool, id: &uuid::Uuid) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        agents::table.find(id).get_result(conn)
    }
}

impl Crud for Agent {
    type Output = Self;
    type SchemaTable = agents::table;
    type CreatePayload = CreateAgentPayload;

    fn schema_table() -> Self::SchemaTable {
        agents::table
    }

    fn create(
        pool: &DbPool,
        #[allow(unused_variables)] uuid: &uuid::Uuid,
        payload: &Self::CreatePayload,
    ) -> QueryResult<Self::Output> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::insert_into(Self::schema_table())
            .values(payload)
            .get_result(conn)
    }
}
