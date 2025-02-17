use crate::db::DbPool;
use crate::users::User;
use diesel::QueryResult;

pub trait LanguageaiSharedUserStorageTrait {
    type Output;
    type CreatePayload;

    fn create(
        pool: &DbPool,
        payload: &Self::CreatePayload,
        user: &User,
        shared_user: &Option<uuid::Uuid>,
    ) -> QueryResult<Self::Output>;
}
