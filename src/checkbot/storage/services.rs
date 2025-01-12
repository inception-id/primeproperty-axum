use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;
use crate::db::DbPool;
use crate::checkbot::services::Checkbot;
use crate::schema::checkbot_storage;

#[derive(Debug, Queryable, Serialize)]
pub(crate) struct CheckbotStorage {
    id: i32,
    user_id: uuid::Uuid,
    checkbot_id:i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    instruction: String,
    content: String,
    updated_completion: String,
}

impl CheckbotStorage {
    pub(super) fn create_checkbot_storage(pool: &DbPool, checkbot: &Checkbot, updated_completion: &str) -> QueryResult<Self> {

        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        
        let values = (
            (checkbot_storage::user_id.eq(&checkbot.user_id)),
            (checkbot_storage::checkbot_id.eq(&checkbot.id)),
            (checkbot_storage::instruction.eq(&checkbot.instruction)),
            (checkbot_storage::content.eq(&checkbot.content)),
            (checkbot_storage::updated_completion.eq(&updated_completion)),
            );
        
        diesel::insert_into(checkbot_storage::table).values(values).get_result(conn)
    }    
}