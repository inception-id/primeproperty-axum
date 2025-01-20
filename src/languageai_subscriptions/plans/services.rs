use crate::db::DbPool;
use crate::languageai_subscriptions::routes::CreateLanguageaiSubscriptionPlansPayload;
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
    name: String,
    initial_price: BigDecimal,
    pub discounted_price: Option<BigDecimal>,
    history_limit: Option<i32>,
    storage_limit: Option<i32>,
    translation_limit: Option<i32>,
    checkbot_limit: Option<i32>,
    text_to_speech_limit: Option<i32>,
    speech_to_text_limit: Option<i32>,
}

impl LanguageaiSubscriptionPlan {
    pub(crate) fn find_all_subscription_plans(pool: &DbPool) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        languageai_subscription_plans::table
            .order_by(languageai_subscription_plans::initial_price.asc())
            .get_results(conn)
    }

    pub(crate) fn create_subscription_plan(
        pool: &DbPool,
        payload: &CreateLanguageaiSubscriptionPlansPayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::insert_into(languageai_subscription_plans::table)
            .values(payload)
            .get_result(conn)
    }

    pub(crate) fn find_subscription_plan_by_id(pool: &DbPool, id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        languageai_subscription_plans::table
            .filter(languageai_subscription_plans::id.eq(id))
            .first(conn)
    }
}
