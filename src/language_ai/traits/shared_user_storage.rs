use crate::db::DbPool;
use crate::language_ai::SharedStoragePermission;
use crate::users::User;
use diesel::{Insertable, QueryResult};

pub trait LanguageaiSharedUserStorageTrait<T: diesel::Table> {
    type Output;
    type CreatePayload: Insertable<T>;

    fn create(
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
}
