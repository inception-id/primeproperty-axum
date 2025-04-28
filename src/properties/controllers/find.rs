use axum::{extract::State, http::HeaderMap};

use crate::{
    agents::Agent,
    db::DbPool,
    middleware::{AxumResponse, JsonResponse, Session},
    properties::model::Property,
};

pub(crate) type PropertyWithAgent = (Property, String, String, Option<String>);

pub async fn find_many_properties(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> AxumResponse<Vec<PropertyWithAgent>> {
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

    match Property::find_many(&pool, &user_id, &role) {
        Ok(property_with_agent) => JsonResponse::send(200, Some(property_with_agent), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}
