use super::routes::CreateCheckbotPayload;
use crate::db::DbPool;
use crate::schema::checkbot;
use crate::utils::get_start_of_month;
use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Checkbot {
    pub id: i32,
    pub user_id: uuid::Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    pub instruction: String,
    ai_system_prompt: String,
    pub content: String,
    pub completion: String,
}

impl Checkbot {
    pub(super) fn create_checkbot(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &CreateCheckbotPayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(checkbot::table)
            .values((checkbot::user_id.eq(user_id), payload))
            .get_result(conn)
    }

    pub(super) fn find_checkbot_history(
        pool: &DbPool,
        user_id: &uuid::Uuid,
    ) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        checkbot::table
            .filter(checkbot::user_id.eq(user_id))
            .order_by(checkbot::created_at.desc())
            .limit(10)
            .get_results(conn)
    }

    pub(super) fn find_checkbot_by_id(pool: &DbPool, checkbot_id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        checkbot::table
            .filter(checkbot::id.eq(&checkbot_id))
            .first(conn)
    }

    pub fn count_current_month_checkbot(pool: &DbPool, user_id: &uuid::Uuid) -> QueryResult<i64> {
        let start_of_month = get_start_of_month();
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        checkbot::table
            .count()
            .filter(
                checkbot::user_id
                    .eq(user_id)
                    .and(checkbot::created_at.gt(start_of_month)),
            )
            .get_result::<i64>(conn)
    }
}
