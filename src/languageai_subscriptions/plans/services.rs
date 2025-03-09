use crate::db::DbPool;
use crate::schema::languageai_subscription_plans;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub(crate) struct LanguageaiSubscriptionPlan {
    pub id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    pub name: String,
    pub initial_price: BigDecimal,
    pub discounted_price: Option<BigDecimal>,
    pub history_limit: Option<i32>,
    pub storage_limit: Option<i32>,
    pub translation_limit: Option<i32>,
    pub checkbot_limit: Option<i32>,
    pub text_to_speech_limit: Option<i32>,
    pub speech_to_text_limit: Option<i32>,
    description: Option<String>,
    category: Option<String>,
    is_active: Option<bool>,
}

impl LanguageaiSubscriptionPlan {
    pub(crate) fn find_all_subscription_plans(pool: &DbPool) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        languageai_subscription_plans::table
            .filter(languageai_subscription_plans::is_active.eq(true))
            .order_by(languageai_subscription_plans::id.asc())
            .get_results(conn)
    }

    pub(crate) fn find_subscription_plan_by_id(pool: &DbPool, id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        languageai_subscription_plans::table
            .filter(languageai_subscription_plans::id.eq(id))
            .first(conn)
    }
}
