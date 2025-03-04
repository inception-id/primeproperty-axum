use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct LanguageAiActiveSubscription {
    pub user_id: uuid::Uuid,
    pub plan_name: String,
    pub expired_at: Option<NaiveDateTime>,
    pub history_limit: Option<i32>,
    pub storage_limit: Option<i32>,
    pub translation_limit: Option<i32>,
    pub translation_count: i64,
    pub translation_storage_count: i64,
    pub checkbot_limit: Option<i32>,
    pub checkbot_count: i64,
    pub checkbot_storage_count: i64,
    pub speech_to_text_limit: Option<i32>,
    pub speech_to_text_count: i64,
    pub speech_to_text_storage_count: i64,
    pub text_to_speech_limit: Option<i32>,
    pub text_to_speech_count: i64,
    pub text_to_speech_storage_count: i64,
}
