use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

use super::agent_role::AgentRole;
use crate::db::DbPool;
use crate::schema::agents;

#[derive(Debug, Serialize, Queryable)]
pub(super) struct Agent {
    id: uuid::Uuid,
    supertokens_user_id: Option<String>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    fullname: String,
    email: String,
    phone_number: String,
    profile_picture_url: Option<String>,
    role: AgentRole,
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
}
