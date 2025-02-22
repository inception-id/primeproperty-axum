use crate::db::DbPool;
use crate::language_ai::{LanguageaiStorageSharing, SharedStoragePermission};
use crate::schema::shared_translation_storage;
use crate::translation::shared_storage::routes::CreateSharedTranslationPayload;
use crate::users::User;
use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
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

impl LanguageaiStorageSharing<shared_translation_storage::table> for SharedTranslationStorage {
    type Output = Self;
    type CreatePayload = CreateSharedTranslationPayload;

    fn create_shared_storage(
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

    fn update_permission(
        pool: &DbPool,
        shared_storage_id: &i32,
        permission: &SharedStoragePermission,
    ) -> QueryResult<Self::Output> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::update(shared_translation_storage::table)
            .set(shared_translation_storage::permission.eq(permission))
            .filter(shared_translation_storage::id.eq(shared_storage_id))
            .get_result(conn)
    }

    fn delete_shared_storage(pool: &DbPool, id: &i32) -> QueryResult<Self::Output> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::delete(
            shared_translation_storage::table.filter(shared_translation_storage::id.eq(id)),
        )
        .get_result(conn)
    }

    fn find_shared_users(pool: &DbPool, storage_id: &i32) -> QueryResult<Vec<Self::Output>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        shared_translation_storage::table
            .filter(shared_translation_storage::translation_storage_id.eq(storage_id))
            .order_by(shared_translation_storage::created_at.desc())
            .get_results(conn)
    }

    fn check_shared_storage_and_shared_email(
        pool: &DbPool,
        storage_id: &i32,
        shared_user_email: &str,
    ) -> QueryResult<Self::Output> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        shared_translation_storage::table
            .filter(
                shared_translation_storage::translation_storage_id
                    .eq(storage_id)
                    .and(shared_translation_storage::shared_user_email.eq(shared_user_email)),
            )
            .get_result(conn)
    }
}
