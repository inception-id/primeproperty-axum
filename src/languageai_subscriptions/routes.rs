use crate::db::DbPool;
use crate::languageai_subscriptions::plans::LanguageaiSubscriptionPlan;
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema::{languageai_subscription_payments, languageai_subscription_plans};

use crate::languageai_subscriptions::enumerates::SubscriptionPeriod;
use crate::languageai_subscriptions::payments::LanguageaiSubscriptionPayment;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use bigdecimal::{BigDecimal, ToPrimitive};
use diesel::Insertable;
use serde::Deserialize;

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
    pub doku_request: serde_json::Value,
    pub doku_response: serde_json::Value,
}

pub(super) async fn create_subscription_payment_checkout_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateLanguageaiSubscriptionPaymentPayload>,
) -> (StatusCode, Json<ApiResponse<LanguageaiSubscriptionPayment>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");

    match LanguageaiSubscriptionPlan::find_subscription_plan_by_id(
        &pool,
        &payload.languageai_subscription_plan_id,
    ) {
        Ok(subscription_plan) => {
            let month_count = &payload.period.clone().to_month_count();
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

pub(super) async fn find_latest_pending_checkout_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<LanguageaiSubscriptionPayment>>) {

    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    
    match LanguageaiSubscriptionPayment::find_latest_pending_checkout(&pool, &user_id) { 
        Ok(pending_checkout) => ApiResponse::new(StatusCode::OK, Some(pending_checkout), "success").send(),
        Err(err) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send(),
    }
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
        ).route(
        "/payment/pending",
        get(find_latest_pending_checkout_route)
    )
}
