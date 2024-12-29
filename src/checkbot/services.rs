use super::routes::CreateCheckbotPayload;
use crate::db::DbPool;
use crate::schema::checkbot;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub(super) struct Checkbot {
    id: i32,
    user_id: uuid::Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    instruction: String,
    ai_system_prompt: String,
    content: String,
    completion: String,
    updated_completion: String,
}

impl Checkbot {
    pub(super) fn create_checkbot(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &CreateCheckbotPayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(checkbot::table)
            .values((checkbot::user_id.eq(user_id), payload))
            .get_result(conn)
    }
}
