use chrono::NaiveDateTime;
use diesel::{QueryResult, Queryable, RunQueryDsl, ExpressionMethods};
use serde::Serialize;
use crate::db::DbPool;
use crate::schema::text_to_speech;
use crate::text_to_speech::routes::CreateTtsPayload;

#[derive(Debug, Queryable, Serialize)]
pub(super) struct TextToSpeech {
    id: i32,
    user_id: uuid::Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    input_content: String,
    audio_url: String,
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
}
