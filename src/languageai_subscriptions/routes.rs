use crate::db::DbPool;
use crate::languageai_subscriptions::enumerates::{PaymentStatus, SubscriptionPeriod};
use crate::languageai_subscriptions::payments::{DokuNotification, LanguageaiSubscriptionPayment};
use crate::languageai_subscriptions::plans::LanguageaiSubscriptionPlan;
use crate::languageai_subscriptions::services::LanguageaiSubscription;
use crate::languageai_subscriptions::SubcriptionLimit;
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema::{languageai_subscription_payments, languageai_subscription_plans};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::Months;
use diesel::Insertable;
use serde::Deserialize;

use super::structs::{LanguageAiActiveSubscription, LanguageaiSubscriptionUsage};

type TPaymentResponse = (StatusCode, Json<ApiResponse<LanguageaiSubscriptionPayment>>);

async fn find_all_subscription_plans_route(
    State(pool): State<DbPool>,
) -> (
    StatusCode,
    Json<ApiResponse<Vec<LanguageaiSubscriptionPlan>>>,
) {
    match LanguageaiSubscriptionPlan::find_all_subscription_plans(&pool) {
        Ok(plans) => ApiResponse::new(StatusCode::OK, Some(plans), "success").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = languageai_subscription_plans)]
pub(crate) struct CreateLanguageaiSubscriptionPlansPayload {
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

async fn create_languageai_subscription_plans_route(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateLanguageaiSubscriptionPlansPayload>,
) -> (StatusCode, Json<ApiResponse<LanguageaiSubscriptionPlan>>) {
    match LanguageaiSubscriptionPlan::create_subscription_plan(&pool, &payload) {
        Ok(plans) => ApiResponse::new(StatusCode::CREATED, Some(plans), "created").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

async fn find_languageai_subscription_plan_by_id_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ApiResponse<LanguageaiSubscriptionPlan>>) {
    match LanguageaiSubscriptionPlan::find_subscription_plan_by_id(&pool, &id) {
        Ok(plan) => ApiResponse::new(StatusCode::OK, Some(plan), "success").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = languageai_subscription_payments)]
pub(super) struct CreateLanguageaiSubscriptionPaymentPayload {
    pub languageai_subscription_plan_id: i32,
    pub period: SubscriptionPeriod,
    pub invoice_id: String,
    pub doku_request: serde_json::Value,
    pub doku_response: serde_json::Value,
}

async fn create_subscription_payment_checkout_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateLanguageaiSubscriptionPaymentPayload>,
) -> TPaymentResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");

    match LanguageaiSubscriptionPlan::find_subscription_plan_by_id(
        &pool,
        &payload.languageai_subscription_plan_id,
    ) {
        Ok(subscription_plan) => {
            let month_count = payload.period.clone().to_month_count() as i32;
            if let Some(plan_discounted_price) = &subscription_plan.discounted_price {
                let amount =
                    BigDecimal::from(&plan_discounted_price.to_i32().unwrap() * month_count);
                let expired_at = chrono::Utc::now().naive_utc() + chrono::Duration::hours(1);
                match LanguageaiSubscriptionPayment::create_checkout(
                    &pool,
                    &user_id,
                    &expired_at,
                    &payload,
                    &amount,
                ) {
                    Ok(payment_checkout) => {
                        ApiResponse::new(StatusCode::CREATED, Some(payment_checkout), "created")
                            .send()
                    }
                    Err(payment_checkout_err) => ApiResponse::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        None,
                        &payment_checkout_err.to_string(),
                    )
                    .send(),
                }
            } else {
                ApiResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    None,
                    "Invalid subscription plan id",
                )
                .send()
            }
        }
        Err(subscription_plan_err) => ApiResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            &subscription_plan_err.to_string(),
        )
        .send(),
    }
}

async fn update_doku_notification_success_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<DokuNotification>,
) -> (StatusCode, Json<ApiResponse<LanguageaiSubscription>>) {
    let header_client_id = headers.get("Client-Id").expect("Missing client id");
    let doku_client_id = &std::env::var("DOKU_CLIENT_ID").expect("missing DOKU_CLIENT_ID");
    if header_client_id != doku_client_id {
        return ApiResponse::new(StatusCode::UNAUTHORIZED, None, "Invalid client id").send();
    }

    let prev_subscription_payment =
        match LanguageaiSubscriptionPayment::find_subscription_payment_by_invoice_id(
            &pool,
            &payload.transaction.original_request_id,
        ) {
            Ok(prev_payment) => prev_payment,
            Err(err) => return ApiResponse::reply(StatusCode::NOT_FOUND, None, &err.to_string()),
        };

    match prev_subscription_payment.status {
        PaymentStatus::Success => ApiResponse::reply(StatusCode::BAD_REQUEST, None, "Paid!"),
        _ => {
            let subscription_payment =
                match LanguageaiSubscriptionPayment::update_doku_notification_success(
                    &pool, &payload,
                ) {
                    Ok(payment) => payment,
                    Err(err) => {
                        return ApiResponse::reply(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            None,
                            &err.to_string(),
                        )
                    }
                };
            let subscription_plan = match LanguageaiSubscriptionPlan::find_subscription_plan_by_id(
                &pool,
                &subscription_payment.languageai_subscription_plan_id,
            ) {
                Ok(plan) => plan,
                Err(err) => {
                    return ApiResponse::reply(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        None,
                        &err.to_string(),
                    )
                }
            };
            let month_count = subscription_payment.period.clone().to_month_count();
            let expired_at = chrono::Utc::now()
                .naive_utc()
                .checked_add_months(Months::new(month_count))
                .expect("Could not add months");
            match LanguageaiSubscription::create_new_subscription(
                &pool,
                &expired_at,
                &subscription_payment,
                &subscription_plan,
            ) {
                Ok(subscription) => ApiResponse::reply(StatusCode::OK, Some(subscription), "ok"),
                Err(subscription_err) => ApiResponse::reply(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    None,
                    &subscription_err.to_string(),
                ),
            }
        }
    }
}

async fn find_user_active_subscription_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<LanguageAiActiveSubscription>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");

    let subscription_usage = match LanguageaiSubscriptionUsage::find_by_user_id(&pool, &user_id) {
        Ok(usage) => usage,
        Err(err) => {
            return ApiResponse::reply(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string())
        }
    };

    let subscription = match LanguageaiSubscription::find_user_active_subscription(&pool, &user_id)
    {
        Ok(sub) => Some(sub),
        Err(_) => None,
    };
    let plan_id = match &subscription {
        Some(sub) => sub.languageai_subscription_plan_id,
        None => 1,
    };

    let subscription_plan =
        match LanguageaiSubscriptionPlan::find_subscription_plan_by_id(&pool, &plan_id) {
            Ok(plan) => plan,
            Err(err) => {
                return ApiResponse::reply(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    None,
                    &err.to_string(),
                )
            }
        };

    match subscription {
        Some(sub) => {
            let active_subscription = LanguageAiActiveSubscription {
                user_id,
                plan_name: subscription_plan.name,
                expired_at: Some(sub.expired_at),
                history_limit: sub.history_limit,
                storage_limit: sub.storage_limit,
                translation_limit: sub.translation_limit,
                translation_count: subscription_usage.translation_count,
                translation_storage_count: subscription_usage.translation_storage_count,
                checkbot_limit: sub.checkbot_limit,
                checkbot_count: subscription_usage.checkbot_count,
                checkbot_storage_count: subscription_usage.checkbot_storage_count,
                text_to_speech_limit: sub.text_to_speech_limit,
                text_to_speech_count: subscription_usage.text_to_speech_count,
                text_to_speech_storage_count: subscription_usage.text_to_speech_storage_count,
                speech_to_text_limit: sub.speech_to_text_limit,
                speech_to_text_count: subscription_usage.speech_to_text_count,
                speech_to_text_storage_count: subscription_usage.speech_to_text_storage_count,
            };
            ApiResponse::reply(StatusCode::OK, Some(active_subscription), "ok")
        }
        None => {
            return ApiResponse::reply(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                "Subscription not found",
            )
        }
    }
}

#[derive(Deserialize)]
struct CheckUserExceedLimitQuery {
    name: SubcriptionLimit,
}

async fn check_user_exceed_subscription_limit_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Query(query): Query<CheckUserExceedLimitQuery>,
) -> (StatusCode, Json<ApiResponse<bool>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    let has_exceed_limit =
        SubcriptionLimit::check_user_exceed_limit(&pool, &user_id, &query.name, &None);
    ApiResponse::new(StatusCode::OK, Some(has_exceed_limit), "success").send()
}

pub fn languageai_subscription_routes() -> Router<DbPool> {
    Router::new()
        .route("/plans", get(find_all_subscription_plans_route))
        .route("/plans", post(create_languageai_subscription_plans_route))
        .route(
            "/plans/:id",
            get(find_languageai_subscription_plan_by_id_route),
        )
        .route(
            "/payment/checkout",
            post(create_subscription_payment_checkout_route),
        )
        .route(
            "/payment/notification/doku",
            post(update_doku_notification_success_route),
        )
        .route("/active", get(find_user_active_subscription_route))
        .route("/limit", get(check_user_exceed_subscription_limit_route))
}
