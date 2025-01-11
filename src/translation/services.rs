use super::routes::CreateTranslationPayload;
use crate::db::DbPool;
use crate::schema::translation;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub(super) struct Translation {
    pub id: i32,
    pub user_id: uuid::Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    ai_system_prompt: String,
    pub content_language: Option<String>,
    pub target_language: String,
    pub content: String,
    completion: String,
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

    pub(super) fn find_translation_history(
        pool: &DbPool,
        user_id: &uuid::Uuid,
    ) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        translation::table
            .filter(translation::user_id.eq(user_id))
            .order_by(translation::id.desc())
            .limit(10)
            .get_results(conn)
    }

    pub(super) fn find_translation(pool: &DbPool, translation_id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        translation::table
            .filter(translation::id.eq(translation_id))
            .first(conn)
    }
}
