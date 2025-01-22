use super::DokuNotification;
use crate::db::DbPool;
use crate::languageai_subscriptions::enumerates::{PaymentStatus, SubscriptionPeriod};
use crate::languageai_subscriptions::routes::CreateLanguageaiSubscriptionPaymentPayload;
use crate::schema::languageai_subscription_payments;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Queryable, Serialize)]
pub(crate) struct LanguageaiSubscriptionPayment {
    id: i32,
    user_id: uuid::Uuid,
    languageai_subscription_plan_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    expired_at: NaiveDateTime,
    amount: BigDecimal,
    period: SubscriptionPeriod,
    pub status: PaymentStatus,
    invoice_id: String,
    doku_request: Option<serde_json::Value>,
    doku_response: Option<serde_json::Value>,
    doku_notification: Option<serde_json::Value>,
}

impl LanguageaiSubscriptionPayment {
    pub(crate) fn create_checkout(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        expired_at: &NaiveDateTime,
        plan: &CreateLanguageaiSubscriptionPaymentPayload,
        amount: &BigDecimal,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let insert_values = (
            languageai_subscription_payments::user_id.eq(user_id),
            languageai_subscription_payments::languageai_subscription_plan_id
                .eq(&plan.languageai_subscription_plan_id),
            languageai_subscription_payments::expired_at.eq(expired_at),
            languageai_subscription_payments::amount.eq(amount),
            languageai_subscription_payments::period.eq(&plan.period),
            languageai_subscription_payments::invoice_id.eq(&plan.invoice_id),
            languageai_subscription_payments::doku_request.eq(&plan.doku_request),
            languageai_subscription_payments::doku_response.eq(&plan.doku_response),
        );

        diesel::insert_into(languageai_subscription_payments::table)
            .values(insert_values)
            .get_result(conn)
    }

    pub(crate) fn find_latest_pending_checkout(
        pool: &DbPool,
        user_id: &uuid::Uuid,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        languageai_subscription_payments::table
            .filter(
                languageai_subscription_payments::user_id
                    .eq(user_id)
                    .and(languageai_subscription_payments::expired_at.gt(diesel::dsl::now)),
            )
            .order_by(languageai_subscription_payments::id.desc())
            .get_result(conn)
    }

    pub(crate) fn find_subscription_payment_by_invoice_id(
        pool: &DbPool,
        invoice_id: &str,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        languageai_subscription_payments::table
            .filter(languageai_subscription_payments::invoice_id.eq(invoice_id))
            .get_result(conn)
    }

    pub(crate) fn update_doku_notification_success(
        pool: &DbPool,
        notification: &DokuNotification,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::update(languageai_subscription_payments::table)
            .filter(
                languageai_subscription_payments::invoice_id
                    .eq(&notification.transaction.original_request_id),
            )
            .set((
                languageai_subscription_payments::status.eq(PaymentStatus::Success),
                languageai_subscription_payments::doku_notification.eq(json!(notification)),
            ))
            .get_result(conn)
    }
}
