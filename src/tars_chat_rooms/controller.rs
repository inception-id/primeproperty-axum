use super::model::TarsChatRoom;
use crate::db::DbPool;
use crate::middleware::extract_header_user_id;
use crate::middleware::ApiResponse;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::{Json, Router};
use reqwest::StatusCode;
use serde::Deserialize;

type TarsChatRoomResponse = (StatusCode, Json<ApiResponse<TarsChatRoom>>);

async fn find_all_tars_chat_rooms_routes(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<TarsChatRoom>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");

    match TarsChatRoom::find_all_by_user_id(&pool, &user_id) {
        Ok(rooms) => ApiResponse::new(StatusCode::OK, Some(rooms), "ok").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct CreateTarsChatRoomPayload {
    pub title: Option<String>,
    pub ai_model_id: i32,
}

async fn create_tars_chat_room_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateTarsChatRoomPayload>,
) -> TarsChatRoomResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");

    match TarsChatRoom::create(&pool, &user_id, &payload) {
        Ok(room) => ApiResponse::new(StatusCode::CREATED, Some(room), "ok").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub fn tars_chat_rooms_routes() -> Router<DbPool> {
    Router::new()
        .route("/find-all", get(find_all_tars_chat_rooms_routes))
        .route("/create", post(create_tars_chat_room_route))
}
