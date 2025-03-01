use crate::schema::tars_chat_rooms;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

use crate::db::DbPool;

use super::controller::CreateTarsChatRoomPayload;

#[derive(Debug, Queryable, Serialize)]
pub(super) struct TarsChatRoom {
    id: i32,
    ai_model_id: i32,
    user_id: uuid::Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    title: Option<String>,
    is_deleted: bool,
}

impl TarsChatRoom {
    pub(super) fn create(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &CreateTarsChatRoomPayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let values = (
            (tars_chat_rooms::ai_model_id.eq(&payload.ai_model_id)),
            (tars_chat_rooms::user_id.eq(user_id)),
            (tars_chat_rooms::title.eq(&payload.title)),
        );

        diesel::insert_into(tars_chat_rooms::table)
            .values(values)
            .get_result(conn)
    }

    pub(super) fn find_all_by_user_id(
        pool: &DbPool,
        user_id: &uuid::Uuid,
    ) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        tars_chat_rooms::table
            .filter(tars_chat_rooms::user_id.eq(user_id))
            .get_results(conn)
    }
}
