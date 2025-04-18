use super::model::Agent;
use crate::{
    db::DbPool,
    middleware::{AxumResponse, JsonResponse},
};
use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::Router;

async fn find_agent_by_supertokens_user_id(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AxumResponse<Agent> {
    match Agent::find_by_supertokens_user_id(&pool, &id) {
        Ok(agent) => JsonResponse::send(200, Some(agent), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub fn agent_routes() -> Router<DbPool> {
    Router::new().route("/supertokens/{id}", get(find_agent_by_supertokens_user_id))
}
