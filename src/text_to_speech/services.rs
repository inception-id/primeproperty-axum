use crate::db::DbPool;
use crate::schema::text_to_speech;
use crate::text_to_speech::routes::CreateTtsPayload;
use crate::utils::get_start_of_month;
use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct TextToSpeech {
    pub id: i32,
    pub user_id: uuid::Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    pub input_content: String,
    pub audio_url: String,
    pub voice: String,
}

impl TextToSpeech {
    pub(super) fn create_tts(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &CreateTtsPayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(text_to_speech::table)
            .values((text_to_speech::user_id.eq(user_id), payload))
            .get_result(conn)
    }

    pub(super) fn find_tts_history(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        history_limit: &Option<i64>,
    ) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        match history_limit {
            Some(limit) => text_to_speech::table
                .filter(text_to_speech::user_id.eq(user_id))
                .order_by(text_to_speech::id.desc())
                .limit(*limit)
                .get_results(conn),
            None => text_to_speech::table
                .filter(text_to_speech::user_id.eq(user_id))
                .order_by(text_to_speech::id.desc())
                .get_results(conn),
        }
    }

    pub(super) fn find_by_id(pool: &DbPool, id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        text_to_speech::table
            .filter(text_to_speech::id.eq(id))
            .get_result(conn)
    }

    pub fn count_current_month_text_to_speech(
        pool: &DbPool,
        user_id: &uuid::Uuid,
    ) -> QueryResult<i64> {
        let start_of_month = get_start_of_month();
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        text_to_speech::table
            .count()
            .filter(
                text_to_speech::user_id
                    .eq(user_id)
                    .and(text_to_speech::created_at.gt(start_of_month)),
            )
            .get_result::<i64>(conn)
    }
}
