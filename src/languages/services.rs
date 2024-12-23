use crate::db::DbPool;
use crate::schema::languages;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;
use crate::languages::routes::UpdateLanguagePayload;

#[derive(Debug, Queryable, Clone, Serialize)]
pub(super) struct Language {
    id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
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

    pub(super) fn find_all_languages(pool: &DbPool) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        languages::table.get_results(conn)
    }
    
    pub(super) fn delete_language(pool: &DbPool, id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        
        diesel::delete(languages::table.filter(languages::id.eq(id))).get_result(conn)
    }
    
    pub(super) fn update_language(pool: &DbPool, id: &i32, payload: &UpdateLanguagePayload) -> QueryResult<Self> {

        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::update(languages::table).filter(languages::id.eq(id)).set(payload).get_result(conn)
    }
}
