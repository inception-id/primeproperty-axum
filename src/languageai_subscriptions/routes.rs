use crate::db::DbPool;
use crate::languageai_subscriptions::plans::LanguageaiSubscriptionPlan;
use crate::middleware::ApiResponse;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};

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

pub fn languageai_subscription_routes() -> Router<DbPool> {
    Router::new().route(
        "/plans",
        get(find_all_subscription_plans_route),
    )
}
