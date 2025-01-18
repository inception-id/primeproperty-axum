use crate::db::DbPool;
use crate::languageai_subscriptions::plans::LanguageaiSubscriptionPlan;
use crate::middleware::ApiResponse;
use crate::schema::languageai_subscription_plans;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use bigdecimal::BigDecimal;
use diesel::{Insertable};
use serde::Deserialize;

pub(crate) async fn find_all_subscription_plans_route(
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

pub(crate) async fn create_languageai_subscription_plans_route(
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

pub(crate) async fn find_languageai_subscription_plan_by_id_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>
) -> (StatusCode, Json<ApiResponse<LanguageaiSubscriptionPlan>>) {
    match LanguageaiSubscriptionPlan::find_subscription_plan_by_id(&pool, &id) { 
        Ok(plan) => ApiResponse::new(StatusCode::OK, Some(plan), "success").send(),
        Err(err) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send(),
    }
}

pub fn languageai_subscription_routes() -> Router<DbPool> {
    Router::new()
        .route("/plans", get(find_all_subscription_plans_route))
        .route("/plans", post(create_languageai_subscription_plans_route))
        .route("/plans/:id", get(find_languageai_subscription_plan_by_id_route))
}
