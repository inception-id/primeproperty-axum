use crate::db::DbPool;
use crate::schema::speech_to_text_storage;
use crate::speech_to_text::services::SpeechToText;
use chrono::{NaiveDateTime};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub(crate) struct SpeechToTextStorage {
    id: i32,
    user_id: uuid::Uuid,
    speech_to_text_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    audio_url: String,
    updated_transcription_text: String,
    language: Option<String>,
}

impl SpeechToTextStorage {
    pub(super) fn create_storage(
        pool: &DbPool,
        speech_to_text: &SpeechToText,
        updated_transcription_text: &str,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        let val = (
            (speech_to_text_storage::user_id.eq(&speech_to_text.user_id)),
            (speech_to_text_storage::speech_to_text_id.eq(&speech_to_text.id)),
            (speech_to_text_storage::audio_url.eq(&speech_to_text.audio_url)),
            (speech_to_text_storage::updated_transcription_text.eq(&updated_transcription_text)),
            (speech_to_text_storage::language.eq(&speech_to_text.language)),
        );

        diesel::insert_into(speech_to_text_storage::table)
            .values(val)
            .get_result(conn)
    }

    pub(super) fn find_storage(pool: &DbPool, user_id: &uuid::Uuid) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        speech_to_text_storage::table
            .filter(speech_to_text_storage::user_id.eq(user_id))
            .limit(10)
            .order_by(speech_to_text_storage::id.desc())
            .get_results(conn)
    }

    pub(super) fn delete_storage(
        pool: &DbPool,
        transcription_storage_id: &i32,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::delete(speech_to_text_storage::table)
            .filter(speech_to_text_storage::id.eq(transcription_storage_id))
            .get_result(conn)
    }

    pub(super) fn update_storage(
        pool: &DbPool,
        transcription_storage_id: &i32,
        updated_transcription_text: &str,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::update(speech_to_text_storage::table)
            .filter(speech_to_text_storage::id.eq(transcription_storage_id))
            .set(speech_to_text_storage::updated_transcription_text.eq(updated_transcription_text))
            .get_result(conn)
    }
}
