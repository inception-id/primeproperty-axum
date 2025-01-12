use crate::checkbot::services::Checkbot;
use crate::db::DbPool;
use crate::schema::checkbot_storage;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub(crate) struct CheckbotStorage {
    id: i32,
    user_id: uuid::Uuid,
    checkbot_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    instruction: String,
    content: String,
    updated_completion: String,
}

impl CheckbotStorage {
    pub(super) fn create_checkbot_storage(
        pool: &DbPool,
        checkbot: &Checkbot,
        updated_completion: &str,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let values = (
            (checkbot_storage::user_id.eq(&checkbot.user_id)),
            (checkbot_storage::checkbot_id.eq(&checkbot.id)),
            (checkbot_storage::instruction.eq(&checkbot.instruction)),
            (checkbot_storage::content.eq(&checkbot.content)),
            (checkbot_storage::updated_completion.eq(&updated_completion)),
        );

        diesel::insert_into(checkbot_storage::table)
            .values(values)
            .get_result(conn)
    }

    pub(super) fn find_many_checkbot_storage(
        pool: &DbPool,
        user_id: &uuid::Uuid,
    ) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        checkbot_storage::table
            .filter(checkbot_storage::user_id.eq(user_id))
            .limit(10)
            .order_by(checkbot_storage::id.desc())
            .get_results(conn)
    }

    pub(super) fn delete_checkbot_storage(
        pool: &DbPool,
        checkbot_storage_id: &i32,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::delete(checkbot_storage::table)
            .filter(checkbot_storage::id.eq(checkbot_storage_id))
            .get_result(conn)
    }

    pub(super) fn update_checkbot_storage(
        pool: &DbPool,
        checkbot_storage_id: &i32,
        updated_completion: &str,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::update(checkbot_storage::table)
            .filter(checkbot_storage::id.eq(checkbot_storage_id))
            .set(checkbot_storage::updated_completion.eq(updated_completion))
            .get_result(conn)
    }
}
