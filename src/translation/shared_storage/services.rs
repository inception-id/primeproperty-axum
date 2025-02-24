use crate::db::DbPool;
use crate::language_ai::{LanguageAiSharedStorageUser, SharedStoragePermission};
use crate::schema::shared_translation_storage;
use crate::translation::shared_storage::join_structs::SharedTranslationStorageJoinTranslationStorage;
use crate::translation::shared_storage::routes::CreateSharedTranslationPayload;
use crate::users::User;
use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct SharedTranslationUser {
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

impl LanguageAiSharedStorageUser<shared_translation_storage::table> for SharedTranslationUser {
    type Output = Self;
    type CreatePayload = CreateSharedTranslationPayload;
    type SharedStorage = SharedTranslationStorageJoinTranslationStorage;

    fn check_shared_user(
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

    fn create_shared_user(
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

    fn update_shared_user_permission(
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

    fn delete_shared_user(pool: &DbPool, id: &i32) -> QueryResult<Self::Output> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::delete(
            shared_translation_storage::table.filter(shared_translation_storage::id.eq(id)),
        )
        .get_result(conn)
    }

    fn find_shared_users(
        pool: &DbPool,
        storage_id: &i32,
        my_email: &str,
    ) -> QueryResult<Vec<Self::Output>> {
        // filter user own email from showing on the list - user should not be able to remove or update their own share storage
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        shared_translation_storage::table
            .filter(
                shared_translation_storage::translation_storage_id
                    .eq(storage_id)
                    .and(shared_translation_storage::shared_user_email.ne(my_email)),
            )
            .order_by(shared_translation_storage::created_at.desc())
            .get_results(conn)
    }

    fn find_shared_storages(
        pool: &DbPool,
        user_id: &uuid::Uuid,
    ) -> QueryResult<Vec<Self::SharedStorage>> {
        let user_id_string = user_id.to_string();
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let sql_query = format!(
            "
            WITH shared_storage AS (
	            SELECT DISTINCT ON (translation_storage_id)
		            id as shared_storage_id,
		            translation_storage_id as storage_id,
		            user_id as owner_id,
		            user_email as owner_email,
		            permission
	            FROM shared_translation_storage
	            WHERE user_id='{user_id_string}'
	            OR shared_user_id='{user_id_string}'
            )
            SELECT
	            shared_storage.shared_storage_id,
	            shared_storage.storage_id,
	            shared_storage.owner_id,
	            shared_storage.owner_email,
	            shared_storage.permission,
	            translation_storage.created_at,
	            translation_storage.updated_at,
	            translation_storage.content_language,
	            translation_storage.target_language,
	            translation_storage.title,
	            translation_storage.content,
	            translation_storage.updated_completion
            FROM shared_storage
            LEFT JOIN translation_storage
                on shared_storage.storage_id = translation_storage.id
            ORDER BY shared_storage.shared_storage_id DESC;
        "
        );

        diesel::sql_query(sql_query).load::<Self::SharedStorage>(conn)
    }
}

impl SharedTranslationUser {
    pub fn upsert_new_id_to_invited_email(
        pool: &DbPool,
        new_id: &uuid::Uuid,
        email: &str,
    ) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::update(shared_translation_storage::table)
            .set(shared_translation_storage::shared_user_id.eq(new_id))
            .filter(shared_translation_storage::shared_user_email.eq(email))
            .get_results(conn)
    }
}
