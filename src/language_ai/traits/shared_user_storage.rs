use crate::db::DbPool;
use crate::language_ai::SharedStoragePermission;
use crate::users::User;
use diesel::{Insertable, QueryResult};

pub trait LanguageaiStorageSharing<T: diesel::Table> {
    type Output;
    type CreatePayload: Insertable<T>;
    type SharedJoinStorageOutput;

    fn check_shared_storage_and_shared_email(
        pool: &DbPool,
        storage_id: &i32,
        shared_user_email: &str,
    ) -> QueryResult<Self::Output>;
    
    fn create_shared_storage(
        pool: &DbPool,
        payload: &Self::CreatePayload,
        user: &User,
        shared_user: &Option<uuid::Uuid>,
    ) -> QueryResult<Self::Output>;

    fn update_storage_permission(
        pool: &DbPool,
        shared_storage_id: &i32,
        permission: &SharedStoragePermission,
    ) -> QueryResult<Self::Output>;

    fn delete_shared_storage(pool: &DbPool, id: &i32) -> QueryResult<Self::Output>;

    fn find_shared_users(pool: &DbPool, storage_id: &i32) -> QueryResult<Vec<Self::Output>>;
    
    fn find_shared_join_storage(pool: &DbPool, user_id: &uuid::Uuid) -> QueryResult<Vec<Self::SharedJoinStorageOutput>>;
}
