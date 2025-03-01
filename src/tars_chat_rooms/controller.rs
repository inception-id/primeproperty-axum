use super::model::TarsChatRoom;
use crate::db::DbPool;
use crate::middleware::extract_header_user_id;
use crate::middleware::ApiResponse;
use crate::tars_chat_messages::CreateTarsChatMessagePayload;
use crate::tars_chat_messages::TarsChatMessage;
use axum::extract::Path;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;

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

#[derive(Deserialize)]
pub(crate) struct CreateTarsChatPayload {
    pub room: CreateTarsChatRoomPayload,
    pub messages: Vec<CreateTarsChatMessagePayload>,
}

#[derive(Serialize)]
pub(crate) struct CreateTarsChatResponse {
    pub room: TarsChatRoom,
    pub messages: Vec<TarsChatMessage>,
}

async fn create_tars_chat_room_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateTarsChatPayload>,
) -> (StatusCode, Json<ApiResponse<CreateTarsChatResponse>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");

    let room = match TarsChatRoom::create(&pool, &user_id, &payload.room) {
        Ok(room) => room,
        Err(err) => {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string())
                .send()
        }
    };

    let messages_with_room_id =
        CreateTarsChatMessagePayload::assign_room_id_to_messages(payload.messages, room.id);
    let messages = match TarsChatMessage::create_multiple(&pool, &messages_with_room_id) {
        Ok(new_messages) => new_messages,
        Err(err) => {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string())
                .send()
        }
    };

    let response = CreateTarsChatResponse { room, messages };

    ApiResponse::new(StatusCode::CREATED, Some(response), "ok").send()
}

async fn delete_tars_chat_room_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> TarsChatRoomResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");

    match TarsChatRoom::delete_chat_room(&pool, &id, &user_id) {
        Ok(room) => ApiResponse::new(StatusCode::OK, Some(room), "ok").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

#[derive(Serialize)]
pub(crate) struct FindTarsChatRoomResponse {
    pub room: TarsChatRoom,
    pub messages: Vec<TarsChatMessage>,
}

async fn find_tars_chat_room_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ApiResponse<FindTarsChatRoomResponse>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");

    match TarsChatRoom::find_by_id(&pool, &id, &user_id) {
        Ok(room) => {
            let messages = match TarsChatMessage::find_by_room_id(&pool, &room.id) {
                Ok(messages) => messages,
                Err(err) => {
                    return ApiResponse::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        None,
                        &err.to_string(),
                    )
                    .send()
                }
            };

            let response = FindTarsChatRoomResponse { room, messages };

            ApiResponse::new(StatusCode::OK, Some(response), "ok").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub fn tars_chat_rooms_routes() -> Router<DbPool> {
    Router::new()
        .route("/find-all", get(find_all_tars_chat_rooms_routes))
        .route("/create", post(create_tars_chat_room_route))
        .route("/delete/:id", delete(delete_tars_chat_room_route))
        .route("/find/:id", get(find_tars_chat_room_route))
}
