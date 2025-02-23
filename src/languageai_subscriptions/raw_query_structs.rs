use crate::db::DbPool;
use diesel::sql_types::{BigInt, Nullable, Uuid};
use diesel::{QueryResult, QueryableByName, RunQueryDsl};
use serde::Serialize;

#[derive(QueryableByName, Debug, Serialize)]
pub(super) struct UserLanguageaiStats {
    #[diesel(sql_type = Uuid)]
    id: uuid::Uuid,
    #[diesel(sql_type = Nullable<BigInt>)]
    translation_count: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    translation_storage_count: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    checkbot_count: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    checkbot_storage_count: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    transcription_count: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    transcription_storage_count: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    tts_count: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    tts_storage_count: Option<i64>,
}

impl UserLanguageaiStats {
    pub(super) fn find_by_user_id(pool: &DbPool, user_id: &uuid::Uuid) -> QueryResult<Vec<Self>> {
        let user_id_string = user_id.to_string();
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let sql_query = format!("
            SELECT 
                users.id,
                t_counts.translation_count,
	            t_store_counts.translation_storage_count,
                cb_counts.checkbot_count,
	            checkbot_storage_count,
                stt_counts.transcription_count,
	            transcription_storage_count,
                tts_counts.tts_count,
	            tts_storage_count
            FROM users
        LEFT JOIN (
            SELECT 
                user_id,
                COUNT(id) AS translation_count
            FROM translation
            WHERE created_at > date_trunc('month', now())
            GROUP BY user_id
        ) AS t_counts ON users.id = t_counts.user_id
        LEFT JOIN (
            SELECT 
                user_id,
                COUNT(id) AS translation_storage_count
            FROM translation_storage
            WHERE created_at > date_trunc('month', now())
            GROUP BY user_id
        ) AS t_store_counts ON users.id = t_store_counts.user_id
        LEFT JOIN (
            SELECT 
                user_id,
                COUNT(id) AS checkbot_count
            FROM checkbot
            WHERE created_at > date_trunc('month', now())
            GROUP BY user_id
        ) AS cb_counts ON users.id = cb_counts.user_id
        LEFT JOIN (
            SELECT 
                user_id,
                COUNT(id) AS checkbot_storage_count
            FROM checkbot_storage
            WHERE created_at > date_trunc('month', now())
            GROUP BY user_id
        ) AS cb_store_counts ON users.id = cb_store_counts.user_id
        LEFT JOIN (
            SELECT 
                user_id,
                SUM(audio_minutes) AS transcription_count
            FROM speech_to_text
            WHERE created_at > date_trunc('month', now())
            GROUP BY user_id
        ) AS stt_counts ON users.id = stt_counts.user_id
        LEFT JOIN (
            SELECT 
                user_id,
                COUNT(id) AS transcription_storage_count
            FROM speech_to_text_storage
            WHERE created_at > date_trunc('month', now())
            GROUP BY user_id
        ) AS stt_storage_counts ON users.id = stt_storage_counts.user_id
        LEFT JOIN (
            SELECT 
                user_id,
                COUNT(id) AS tts_count
            FROM text_to_speech
            WHERE created_at > date_trunc('month', now())
            GROUP BY user_id
        ) AS tts_counts ON users.id = tts_counts.user_id
        LEFT JOIN (
            SELECT 
                user_id,
                COUNT(id) AS tts_storage_count
            FROM text_to_speech_storage
            WHERE created_at > date_trunc('month', now())
            GROUP BY user_id
        ) AS tts_storage_counts ON users.id = tts_storage_counts.user_id
        WHERE users.id = '{user_id_string}';");

        diesel::sql_query(sql_query).load::<UserLanguageaiStats>(conn)
    }
}
