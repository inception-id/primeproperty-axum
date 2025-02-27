use crate::db::DbPool;
use crate::language_ai::SharedStoragePermission;
use crate::users::User;
use diesel::{Insertable, QueryResult};

pub trait LanguageAiSharedStorageUser<T: diesel::Table> {
    type Output;
    type CreatePayload: Insertable<T>;
    type SharedStorage;

    fn check_shared_user(
        pool: &DbPool,
        storage_id: &i32,
        shared_user_email: &str,
    ) -> QueryResult<Self::Output>;

    fn create_shared_user(
        pool: &DbPool,
        payload: &Self::CreatePayload,
        user: &User,
        shared_user: &Option<uuid::Uuid>,
    ) -> QueryResult<Self::Output>;

    fn update_shared_user_permission(
        pool: &DbPool,
        shared_storage_id: &i32,
        permission: &SharedStoragePermission,
    ) -> QueryResult<Self::Output>;

    fn delete_shared_user(pool: &DbPool, id: &i32) -> QueryResult<Self::Output>;

    fn find_shared_users(
        pool: &DbPool,
        storage_id: &i32,
        my_email: &str,
    ) -> QueryResult<Vec<Self::Output>>;

    fn find_shared_storages(
        pool: &DbPool,
        user_id: &uuid::Uuid,
    ) -> QueryResult<Vec<Self::SharedStorage>>;

    fn update_invited_email_user_id(
        pool: &DbPool,
        new_id: &uuid::Uuid,
        email: &str,
    ) -> QueryResult<Vec<Self::Output>>;
}
