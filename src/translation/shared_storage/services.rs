use crate::db::DbPool;
use crate::language_ai::{LanguageaiSharedUserStorageTrait, SharedStoragePermission};
use crate::schema::shared_translation_storage;
use crate::translation::shared_storage::routes::CreateSharedTranslationPayload;
use crate::users::User;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub(crate) struct SharedTranslationStorage {
    id: i32,
    user_id: uuid::Uuid,
    shared_user_id: Option<uuid::Uuid>,
    translation_storage_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    user_email: String,
    shared_user_email: String,
    permission: SharedStoragePermission,
}

impl LanguageaiSharedUserStorageTrait for SharedTranslationStorage {
    type Output = Self;
    type CreatePayload = CreateSharedTranslationPayload;

    fn create(
        pool: &DbPool,
        payload: &Self::CreatePayload,
        user: &User,
        shared_user_id: &Option<uuid::Uuid>,
    ) -> QueryResult<Self::Output> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let data = (
            shared_translation_storage::user_id.eq(&user.id),
            shared_translation_storage::user_email.eq(&user.email),
            shared_translation_storage::shared_user_id.eq(shared_user_id),
            payload,
        );
        diesel::insert_into(shared_translation_storage::table)
            .values(data)
            .get_result(conn)
    }
}
