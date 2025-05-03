use crate::middleware::JsonResponse;
use crate::properties::Property;
use crate::traits::Crud;
use crate::{db::DbPool, middleware::AxumResponse, schema};
use axum::extract::{Json, State};
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
    Json(payload): Json<CreateLeadPayload>,
) -> AxumResponse<Lead> {
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
