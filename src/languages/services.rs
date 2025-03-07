use crate::db::DbPool;
use crate::schema::languages;
use chrono::NaiveDateTime;
use diesel::{QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Queryable, Clone, Serialize)]
pub(super) struct Language {
    id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    title: String,
    iso_639_1: String,
}

impl Language {
    pub(super) fn find_all_languages(pool: &DbPool) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        languages::table
            .order_by(languages::title)
            .get_results(conn)
    }
}
