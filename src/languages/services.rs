use crate::db::DbPool;
use crate::schema::languages;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Queryable, Clone, Serialize)]
pub(super) struct Language {
    id: i32,
    created_date: NaiveDateTime,
    updated_date: NaiveDateTime,
    title: String,
    iso_639_1: String,
}

impl Language {
    pub(super) fn create_language(
        pool: &DbPool,
        title: &str,
        iso_639_1: &str,
    ) -> QueryResult<Self> {
        let data = (
            languages::title.eq(title.trim().to_lowercase()),
            languages::iso_639_1.eq(iso_639_1.trim().to_lowercase()),
        );

        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(languages::table)
            .values(data)
            .get_result(conn)
    }
}
