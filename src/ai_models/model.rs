use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

use crate::{db::DbPool, schema::ai_models};

use super::controller::CreateAiModelPayload;

#[derive(Debug, Queryable, Serialize)]
pub(super) struct AiModel {
    id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    parent: String,
    label: String,
    value: String,
}

impl AiModel {
    pub(super) fn create(pool: &DbPool, payload: &CreateAiModelPayload) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::insert_into(ai_models::table)
            .values(payload)
            .get_result(conn)
    }
    pub(super) fn find_all(pool: &DbPool) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        ai_models::table
            .order_by(ai_models::parent.desc())
            .order_by(ai_models::label.desc())
            .get_results(conn)
    }
}
