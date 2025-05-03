use crate::middleware::JsonResponse;
use crate::properties::Property;
use crate::traits::Crud;
use crate::{db::DbPool, middleware::AxumResponse, schema};
use axum::extract::{Json, State};
use axum::http::HeaderMap;
use axum::routing::post;
use axum::Router;
use diesel::prelude::Insertable;
use serde::Deserialize;

use super::model::Lead;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::leads)]
pub struct CreateLeadPayload {
    user_id: uuid::Uuid,
    property_id: i32,
    name: String,
    phone: String,
    email: Option<String>,
}

async fn create_lead(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateLeadPayload>,
) -> AxumResponse<Lead> {
    let api_key_option = headers.get("x-api-key");

    let api_key = match api_key_option {
        Some(key) => key.to_str().unwrap_or(""),
        None => return JsonResponse::send(401, None, None),
    };

    let leads_api_key = std::env::var("API_KEY_LEADS").expect("Missing API_KEY_LEADS");

    if api_key != leads_api_key {
        return JsonResponse::send(401, None, None);
    }

    let property = match Property::find_one_by_id(&pool, &payload.property_id) {
        Ok(property) if property.0.user_id == payload.user_id => property,
        _ => return JsonResponse::send(400, None, None),
    };
    match Lead::create(&pool, &property.0.user_id, &payload) {
        Ok(lead) => JsonResponse::send(201, Some(lead), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub fn lead_routes() -> Router<DbPool> {
    Router::new().route("/", post(create_lead))
}
