use crate::db::DbPool;
use crate::schema::translation_storage;
use crate::translation::services::Translation;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;
use crate::middleware::StorageVisibility;
use super::routes::{CreateTranslationStoragePayload, UpdateTranslationStoragePayload};

#[derive(Debug, Queryable, Serialize)]
pub struct TranslationStorage {
    id: i32,
    user_id: uuid::Uuid,
    translation_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    content_language: Option<String>,
    target_language: String,
    content: String,
    updated_completion: String,
    title: Option<String>,
    visibility: StorageVisibility
}

impl TranslationStorage {
    pub(super) fn create_translation_storage(
        pool: &DbPool,
        translation: &Translation,
        payload: &CreateTranslationStoragePayload
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let values = (
            (translation_storage::user_id.eq(&translation.user_id)),
            // (translation_storage::translation_id.eq(&translation.id)),
            (translation_storage::content_language.eq(&translation.content_language)),
            (translation_storage::target_language.eq(&translation.target_language)),
            (translation_storage::content.eq(&translation.content)),
            payload
            // (translation_storage::updated_completion.eq(payload.title.clone().unwrap())),
            // (translation_storage::updated_completion.eq(&payload.updated_completion)),
        );

        diesel::insert_into(translation_storage::table)
            .values(values)
            .get_result(conn)
    }

    pub(super) fn find_user_translation_storage(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        storage_limit: &Option<i64>,
    ) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        match storage_limit {
            Some(limit) => translation_storage::table
                .filter(translation_storage::user_id.eq(user_id))
                .limit(*limit)
                .order_by(translation_storage::id.desc())
                .get_results(conn),
            None => translation_storage::table
                .filter(translation_storage::user_id.eq(user_id))
                .order_by(translation_storage::id.desc())
                .get_results(conn),
        }
    }

    pub(super) fn delete_translation_storage(
        pool: &DbPool,
        translation_id: &i32,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::delete(
            translation_storage::table.filter(translation_storage::id.eq(translation_id)),
        )
        .get_result(conn)
    }

    pub(super) fn update_translation_storage(
        pool: &DbPool,
        translation_id: &i32,
        // updated_completion: &str,
        payload: &UpdateTranslationStoragePayload
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::update(translation_storage::table)
            .filter(translation_storage::id.eq(translation_id))
            .set(payload)
            .get_result(conn)
    }

    pub fn count_user_translation_storage(pool: &DbPool, user_id: &uuid::Uuid) -> QueryResult<i64> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        translation_storage::table
            .count()
            .filter(translation_storage::user_id.eq(user_id))
            .get_result(conn)
    }
}
