use super::routes::CreateCheckbotPayload;
use crate::schema::checkbot;
use crate::utils::get_start_of_month;
use crate::{db::DbPool, language_ai::LanguageAiCrud};
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
    input_tokens: i32,
    output_tokens: i32,
    total_tokens: i32,
    temperature: f64,
}

impl LanguageAiCrud for Checkbot {
    type Output = Self;
    type CreatePayload = CreateCheckbotPayload;

    fn create(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &Self::CreatePayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(checkbot::table)
            .values((checkbot::user_id.eq(user_id), payload))
            .get_result(conn)
    }
}

impl Checkbot {
    pub(super) fn find_checkbot_history(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        history_limit: &Option<i64>,
    ) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        match history_limit {
            Some(limit) => checkbot::table
                .filter(checkbot::user_id.eq(user_id))
                .order_by(checkbot::created_at.desc())
                .limit(*limit)
                .get_results(conn),
            None => checkbot::table
                .filter(checkbot::user_id.eq(user_id))
                .order_by(checkbot::created_at.desc())
                .get_results(conn),
        }
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
