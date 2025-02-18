use crate::db::DbPool;
use crate::language_ai::SharedStoragePermission;
use crate::users::User;
use diesel::{Insertable, QueryResult};

pub trait LanguageaiStorageSharing<T: diesel::Table> {
    type Output;
    type CreatePayload: Insertable<T>;

    fn create_shared_storage(
        pool: &DbPool,
        payload: &Self::CreatePayload,
        user: &User,
        shared_user: &Option<uuid::Uuid>,
    ) -> QueryResult<Self::Output>;

    fn update_permission(
        pool: &DbPool,
        shared_storage_id: &i32,
        permission: &SharedStoragePermission,
    ) -> QueryResult<Self::Output>;

    fn delete_shared_storage(pool: &DbPool, id: &i32) -> QueryResult<Self::Output>;
}
