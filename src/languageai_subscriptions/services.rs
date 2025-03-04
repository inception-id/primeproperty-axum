use crate::db::DbPool;
use crate::languageai_subscriptions::payments::LanguageaiSubscriptionPayment;
use crate::languageai_subscriptions::plans::LanguageaiSubscriptionPlan;
use crate::schema::languageai_subscriptions;
use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct LanguageaiSubscription {
    id: i32,
    user_id: uuid::Uuid,
    pub languageai_subscription_plan_id: i32,
    languageai_subscription_payment_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    expired_at: NaiveDateTime,
    pub history_limit: Option<i32>,
    pub storage_limit: Option<i32>,
    pub translation_limit: Option<i32>,
    pub checkbot_limit: Option<i32>,
    pub text_to_speech_limit: Option<i32>,
    pub speech_to_text_limit: Option<i32>,
}

impl LanguageaiSubscription {
    pub(super) fn create_new_subscription(
        pool: &DbPool,
        expired_at: &NaiveDateTime,
        subscription_payment: &LanguageaiSubscriptionPayment,
        subscription_plan: &LanguageaiSubscriptionPlan,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let values = (
            languageai_subscriptions::user_id.eq(&subscription_payment.user_id),
            languageai_subscriptions::languageai_subscription_plan_id
                .eq(&subscription_payment.languageai_subscription_plan_id),
            languageai_subscriptions::languageai_subscription_payment_id
                .eq(&subscription_payment.id),
            languageai_subscriptions::expired_at.eq(&expired_at),
            languageai_subscriptions::history_limit.eq(subscription_plan.history_limit),
            languageai_subscriptions::storage_limit.eq(subscription_plan.storage_limit),
            languageai_subscriptions::translation_limit.eq(subscription_plan.translation_limit),
            languageai_subscriptions::checkbot_limit.eq(subscription_plan.checkbot_limit),
            languageai_subscriptions::text_to_speech_limit
                .eq(subscription_plan.text_to_speech_limit),
            languageai_subscriptions::speech_to_text_limit
                .eq(subscription_plan.speech_to_text_limit),
        );

        diesel::insert_into(languageai_subscriptions::table)
            .values(values)
            .get_result(conn)
    }

    pub fn find_user_active_subscription(pool: &DbPool, user_id: &uuid::Uuid) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        languageai_subscriptions::table
            .filter(
                languageai_subscriptions::user_id
                    .eq(user_id)
                    .and(languageai_subscriptions::expired_at.gt(diesel::dsl::now)),
            )
            .order_by(languageai_subscriptions::id.desc())
            .get_result(conn)
    }
}
