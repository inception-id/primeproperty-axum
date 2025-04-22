use super::model::Agent;
use crate::middleware::{JsonFindResponse, Role, Session};
use crate::traits::Crud;
use crate::{
    db::DbPool,
    middleware::{AxumResponse, JsonResponse},
    schema,
};
use axum::extract::{Json, Path, Query, State};
use axum::http::HeaderMap;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post, put};
use axum::Router;
use diesel::prelude::{AsChangeset, Insertable};
use serde::Deserialize;

async fn find_agent_by_supertokens_user_id(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AxumResponse<Agent> {
    match Agent::find_by_supertokens_user_id(&pool, &id) {
        Ok(agent) => JsonResponse::send(200, Some(agent), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::agents)]
pub struct CreateAgentPayload {
    supertokens_user_id: String,
    fullname: String,
    email: String,
    phone_number: String,
    profile_picture_url: Option<String>,
}

async fn create_agent(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateAgentPayload>,
) -> AxumResponse<Agent> {
    let user_id = Session::extract_session_user_id(&headers);

    // test this
    match Agent::find_by_email(&pool, &payload.email) {
        Ok(_) => JsonResponse::send(400, None, Some("Email already exists".to_string())),
        _ => match Agent::create(&pool, &user_id, &payload) {
            Ok(agent) => JsonResponse::send(201, Some(agent), None),
            Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
        },
    }
}

#[derive(Deserialize)]
pub struct FindAgentQuery {
    pub name_or_email: Option<String>,
    pub page: Option<i64>,
}
async fn find_agents(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Query(query): Query<FindAgentQuery>,
) -> AxumResponse<JsonFindResponse<Vec<Agent>>> {
    let user_id = Session::extract_session_user_id(&headers);
    let agents = match Agent::find(&pool, &user_id, &query) {
        Ok(agents) => agents,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    let total_agent_pages = match Agent::count_find_total(&pool, &user_id, &query) {
        Ok(agents_count) => (agents_count / Agent::PAGE_SIZE) + 1,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    JsonResponse::send(
        200,
        Some(JsonFindResponse {
            data: agents,
            total_pages: total_agent_pages,
        }),
        None,
    )
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = schema::agents)]
pub struct UpdateAgentPayload {
    profile_picture_url: Option<String>,
    fullname: Option<String>,
    phone_number: Option<String>,
}

// for agents to update their information themselves
async fn update_agent(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<UpdateAgentPayload>,
) -> AxumResponse<Agent> {
    let user_id = Session::extract_session_user_id(&headers);

    match Agent::update_agent(&pool, &user_id, &payload) {
        Ok(agent) => JsonResponse::send(200, Some(agent), None),
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    }
}

// for admin to update their agent information
async fn update_agent_from_admin(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAgentPayload>,
) -> AxumResponse<Agent> {
    let agent_id = uuid::Uuid::parse_str(&id).expect("Invalid agent id");
    match Agent::update_agent(&pool, &agent_id, &payload) {
        Ok(agent) => JsonResponse::send(200, Some(agent), None),
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    }
}

async fn delete_agent(State(pool): State<DbPool>, Path(id): Path<String>) -> AxumResponse<Agent> {
    let agent_id = uuid::Uuid::parse_str(&id).expect("Invalid agent id");
    match Agent::delete_agent(&pool, &agent_id) {
        Ok(agent) => JsonResponse::send(200, Some(agent), None),
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub fn agent_routes(pool: DbPool) -> Router<DbPool> {
    Router::new()
        .route("/", post(create_agent))
        .route("/", get(find_agents))
        .route("/{id}", put(update_agent_from_admin))
        .route("/{id}", delete(delete_agent))
        .layer(from_fn_with_state(pool.clone(), Role::middleware))
        .route("/", put(update_agent))
        .route("/supertokens/{id}", get(find_agent_by_supertokens_user_id))
}
