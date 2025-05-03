use crate::agents::{Agent, AgentRole};
use crate::leads::Lead;
use crate::middleware::Session;
use crate::properties::model::Property;
use crate::{
    db::DbPool,
    middleware::{AxumResponse, JsonResponse},
};
use axum::extract::Path;
use axum::{extract::State, http::HeaderMap};

pub async fn delete_property(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> AxumResponse<Property> {
    let user_id = Session::extract_session_user_id(&headers);

    let role = match Agent::find_by_user_id(&pool, &user_id) {
        Ok(agent) => agent.role,
        Err(err) => return JsonResponse::send(401, None, Some(err.to_string())),
    };

    let is_admin = match role {
        AgentRole::Admin => true,
        AgentRole::Agent => false,
    };

    let property = match Property::find_one_by_id(&pool, &id) {
        Ok(property) => property,
        Err(err) => return JsonResponse::send(400, None, Some(err.to_string())),
    };

    if property.0.user_id != user_id && !is_admin {
        return JsonResponse::send(403, None, Some("Forbidden".to_string()));
    }

    // Update leads is_deleted to true
    if let AgentRole::Agent = role {
        let _ = Lead::delete_by_property_id(&pool, &property.0.id);
    }

    match Property::delete(&pool, &id, &role) {
        Ok(property) => JsonResponse::send(200, Some(property), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}
