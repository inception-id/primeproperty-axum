use crate::schema::tars_chat_rooms;
use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

use crate::db::DbPool;

#[derive(Debug, Queryable, Serialize)]
pub(super) struct TarsChatRoom {
    pub id: i32,
    ai_model_id: i32,
    user_id: uuid::Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    title: Option<String>,
    is_temporary: bool,
}

impl TarsChatRoom {
    pub(super) fn create(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        ai_model_id: &i32,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let values = (
            (tars_chat_rooms::ai_model_id.eq(&ai_model_id)),
            (tars_chat_rooms::user_id.eq(user_id)),
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
            .order_by(tars_chat_rooms::id.desc())
            .get_results(conn)
    }

    pub(super) fn delete_chat_room(
        pool: &DbPool,
        id: &i32,
        user_id: &uuid::Uuid,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::update(tars_chat_rooms::table)
            .filter(
                tars_chat_rooms::id
                    .eq(id)
                    .and(tars_chat_rooms::user_id.eq(user_id)),
            )
            .set(tars_chat_rooms::is_temporary.eq(true))
            .get_result(conn)
    }

    pub(super) fn find_by_id(pool: &DbPool, id: &i32, user_id: &uuid::Uuid) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        tars_chat_rooms::table
            .filter(
                tars_chat_rooms::id
                    .eq(id)
                    .and(tars_chat_rooms::user_id.eq(user_id)),
            )
            .get_result(conn)
    }
}
