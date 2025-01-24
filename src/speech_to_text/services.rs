use super::routes::CreateTranscriptionPayload;
use crate::db::DbPool;
use crate::schema::speech_to_text;
use crate::utils::get_start_of_month;
use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct SpeechToText {
    pub id: i32,
    pub user_id: uuid::Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    pub audio_url: String,
    pub transcription_text: String,
    pub language: Option<String>,
}

impl SpeechToText {
    pub(super) fn create_transcription(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &CreateTranscriptionPayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(speech_to_text::table)
            .values((speech_to_text::user_id.eq(user_id), payload))
            .get_result(conn)
    }

    pub(super) fn find_transcription_history(
        pool: &DbPool,
        user_id: &uuid::Uuid,
    ) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        speech_to_text::table
            .filter(speech_to_text::user_id.eq(user_id))
            .order_by(speech_to_text::id.desc())
            .limit(10)
            .get_results(conn)
    }

    pub(super) fn find_transcription_by_id(pool: &DbPool, id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        speech_to_text::table
            .filter(speech_to_text::id.eq(id))
            .first(conn)
    }

    pub fn count_current_month_speech_to_text(
        pool: &DbPool,
        user_id: &uuid::Uuid,
    ) -> QueryResult<i64> {
        let start_of_month = get_start_of_month();
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        speech_to_text::table
            .count()
            .filter(
                speech_to_text::user_id
                    .eq(user_id)
                    .and(speech_to_text::created_at.gt(start_of_month)),
            )
            .get_result::<i64>(conn)
    }
}
