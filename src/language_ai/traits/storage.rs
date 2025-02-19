use crate::db::DbPool;
use diesel::QueryResult;

pub trait LanguageaiStorage {
    type Output;

    fn find_storage_by_id(pool: &DbPool, storage_id: &i32) -> QueryResult<Self::Output>;
}
