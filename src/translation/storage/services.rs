use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Queryable, RunQueryDsl, QueryResult};
use serde::Serialize;
use crate::db::DbPool;
use crate::translation::services::Translation;
use crate::schema::translation_storage;

#[derive(Debug, Queryable, Serialize)]
pub(crate) struct TranslationStorage {
    id: i32,
    user_id: uuid::Uuid,
    translation_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    content_language: Option<String>,
    target_language: String,
    content: String,
    updated_completion: String,
}

impl TranslationStorage {
    pub(super) fn create_translation_storage(
        pool: &DbPool,
        translation: &Translation,
        updated_completion: &str,
    ) -> QueryResult<Self>{
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let values = (
            (translation_storage::user_id.eq(&translation.user_id)),
            (translation_storage::translation_id.eq(&translation.id)),
            (translation_storage::content_language.eq(&translation.content_language)),
            (translation_storage::target_language.eq(&translation.target_language)),
            (translation_storage::content.eq(&translation.content)),
            (translation_storage::updated_completion.eq(&updated_completion)),
        );

        diesel::insert_into(translation_storage::table).values(values).get_result(conn)
    }
}

