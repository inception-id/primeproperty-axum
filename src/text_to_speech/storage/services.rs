use crate::db::DbPool;
use crate::schema::text_to_speech_storage;
use crate::text_to_speech::services::TextToSpeech;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct TextToSpeechStorage {
    id: i32,
    user_id: uuid::Uuid,
    text_to_speech_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    input_content: String,
    audio_url: String,
    voice: String,
}

impl TextToSpeechStorage {
    pub(super) fn create_tts_storage(pool: &DbPool, tts: &TextToSpeech) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        let val = (
            text_to_speech_storage::user_id.eq(&tts.user_id),
            text_to_speech_storage::text_to_speech_id.eq(&tts.id),
            text_to_speech_storage::input_content.eq(&tts.input_content),
            text_to_speech_storage::audio_url.eq(&tts.audio_url),
            text_to_speech_storage::voice.eq(&tts.voice),
        );

        diesel::insert_into(text_to_speech_storage::table)
            .values(val)
            .get_result(conn)
    }

    pub(super) fn find_many_tts_storage(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        storage_limit: &Option<i64>,
    ) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        match storage_limit {
            Some(limit) => text_to_speech_storage::table
                .filter(text_to_speech_storage::user_id.eq(user_id))
                .limit(*limit)
                .order_by(text_to_speech_storage::id.desc())
                .get_results(conn),
            None => text_to_speech_storage::table
                .filter(text_to_speech_storage::user_id.eq(user_id))
                .order_by(text_to_speech_storage::id.desc())
                .get_results(conn),
        }
    }

    pub(super) fn delete_tts_storage(pool: &DbPool, tts_storage_id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::delete(text_to_speech_storage::table)
            .filter(text_to_speech_storage::id.eq(tts_storage_id))
            .get_result(conn)
    }

    pub fn count_tts_storage(pool: &DbPool, user_id: &uuid::Uuid) -> QueryResult<i64> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        text_to_speech_storage::table
            .count()
            .filter(text_to_speech_storage::user_id.eq(user_id))
            .get_result(conn)
    }
}
