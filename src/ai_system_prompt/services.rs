use crate::db::DbPool;
use crate::schema::ai_system_prompts;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(super) struct AiSystemPrompt {
    id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    product_name: String,
    prompt: String,
    name: String,
}

impl AiSystemPrompt {
    pub(super) fn find_ai_system_prompts(
        pool: &DbPool,
        product_name: &Option<String>,
    ) -> QueryResult<Vec<AiSystemPrompt>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        if let Some(product_name) = product_name {
            ai_system_prompts::table
                .filter(ai_system_prompts::product_name.eq(product_name))
                .get_results(conn)
        } else {
            ai_system_prompts::table.get_results(conn)
        }
    }
}
