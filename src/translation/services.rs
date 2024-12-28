use super::routes::CreateTranslationPayload;
use crate::db::DbPool;
use crate::schema::{translation};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub(super) struct Translation {
    id: i32,
    user_id: uuid::Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    ai_system_prompt: String,
    content_language: Option<String>,
    target_language: String,
    content: String,
    completion: String,
    updated_completion: String,
}

impl Translation {
    pub(super) fn create_translation(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &CreateTranslationPayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(translation::table)
            .values((translation::user_id.eq(user_id), payload))
            .get_result(conn)
    }
    
    pub(super) fn update_translation(
        pool: &DbPool,
        id: &i32,
        updated_completion: &str
    ) -> QueryResult<Self> {

        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::update(translation::table)
            .filter(translation::id.eq(id))
            .set(translation::updated_completion.eq(updated_completion))
            .get_result(conn)
    }
}
