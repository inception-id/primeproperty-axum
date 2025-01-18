use crate::db::DbPool;
use crate::schema::languageai_subscription_plans;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, Serialize)]
#[diesel(table_name = languageai_subscription_plans)]
pub(crate) struct LanguageaiSubscriptionPlan {
    id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    name: String,
    initial_price: BigDecimal,
    discounted_price: Option<BigDecimal>,
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
        languageai_subscription_plans::table.get_results(conn)
    }
}
