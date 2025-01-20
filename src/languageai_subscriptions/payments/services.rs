use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{QueryResult, Queryable, ExpressionMethods, RunQueryDsl};
use serde::Serialize;
use crate::db::DbPool;
use crate::languageai_subscriptions::enumerates::{PaymentStatus, SubscriptionPeriod};
use crate::languageai_subscriptions::routes::CreateLanguageaiSubscriptionPaymentPayload;
use crate::schema::languageai_subscription_payments;

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
    status: PaymentStatus,
    doku_response: Option<serde_json::Value>
}

impl LanguageaiSubscriptionPayment {
   pub(crate) fn create_checkout(
       pool: &DbPool,
       user_id: &uuid::Uuid,
       expired_at: &NaiveDateTime,
       plan: &CreateLanguageaiSubscriptionPaymentPayload,
       amount: &BigDecimal
   ) -> QueryResult<Self> {

       let conn = &mut pool.get().expect("Couldn't get db connection from pool");
      
       let insert_values = (
           languageai_subscription_payments::user_id.eq(user_id),
           languageai_subscription_payments::expired_at.eq(expired_at),
           languageai_subscription_payments::languageai_subscription_plan_id.eq(&plan.languageai_subscription_plan_id),
           languageai_subscription_payments::period.eq(&plan.period),
           languageai_subscription_payments::amount.eq(amount),
       );
       
       diesel::insert_into(languageai_subscription_payments::table).values(insert_values).get_result(conn)
   }
}