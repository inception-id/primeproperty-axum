use crate::db::DbPool;
use diesel::QueryResult;

pub trait LanguageAiCrud {
    type Output;
    type CreatePayload;

    fn create(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &Self::CreatePayload,
    ) -> QueryResult<Self::Output>;
}
