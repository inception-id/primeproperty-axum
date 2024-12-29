use chrono::NaiveDateTime;
use diesel::{QueryResult, Queryable, RunQueryDsl, ExpressionMethods};
use serde::Serialize;
use crate::db::DbPool;
use crate::schema::speech_to_text;
use super::routes::CreateTranscriptionPayload;

#[derive(Debug, Queryable, Serialize)]
pub(super) struct SpeechToText {
    id: i32,
    user_id: uuid::Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    audio_url: Option<String>,
    transcription_text: String, 
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
}