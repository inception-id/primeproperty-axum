use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
};
use serde::Deserialize;

use crate::{
    agents::Agent,
    db::DbPool,
    middleware::{AxumResponse, JsonFindResponse, JsonResponse, Session},
    properties::model::Property,
    traits::PAGE_SIZE,
};

#[derive(Deserialize)]
pub struct FindPropertyQuery {
    pub s: Option<String>,
    pub page: Option<i64>,
}

pub(crate) type PropertyWithAgent = (Property, String, String, Option<String>);

pub async fn find_many_properties(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Query(query): Query<FindPropertyQuery>,
) -> AxumResponse<JsonFindResponse<Vec<PropertyWithAgent>>> {
    let header_user_id = headers.get("x-user-id");
    let (user_id, role) = match header_user_id {
        Some(_) => {
            let user_id = Session::extract_session_user_id(&headers);
            match Agent::find_by_user_id(&pool, &user_id) {
                Ok(agent) => (Some(user_id), Some(agent.role)),
                _ => {
                    return JsonResponse::send(403, None, None);
                }
            }
        }
        None => (None, None),
    };

    let property_with_agent = match Property::find_many(&pool, &user_id, &role, &query) {
        Ok(property_with_agent) => property_with_agent,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    let total_property_pages = match Property::count_find_many_total(&pool, &user_id, &role) {
        Ok(property_with_agent_count) => (property_with_agent_count / PAGE_SIZE) + 1,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    JsonResponse::send(
        200,
        Some(JsonFindResponse {
            data: property_with_agent,
            total_pages: total_property_pages,
        }),
        None,
    )
}

pub async fn find_one_by_id(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> AxumResponse<PropertyWithAgent> {
    match Property::find_one_by_id(&pool, &id) {
        Ok(property) => JsonResponse::send(200, Some(property), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}
