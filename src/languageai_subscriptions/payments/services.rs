use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;
use crate::languageai_subscriptions::enumerates::{PaymentStatus, SubscriptionPeriod};

#[derive(Queryable, Serialize)]
pub(crate) struct LanguageaiSubscriptionPayment {
    id: i32,
    user_id: uuid::Uuid,
    languageai_subscription_plan_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    expired_at: NaiveDateTime,
    amount: BigDecimal,
    period: SubscriptionPeriod,
    status: PaymentStatus,
    doku_response: Option<serde_json::Value>
}

