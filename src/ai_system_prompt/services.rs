use crate::db::DbPool;
use crate::schema::ai_system_prompts;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(super) struct AiSystemPrompt {
    id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    product_name: String,
    prompt: String,
}

impl AiSystemPrompt {
    pub(super) fn create_ai_system_prompt(
        pool: &DbPool,
        product_name: &str,
        prompt: &str,
    ) -> QueryResult<AiSystemPrompt> {
        let data = (
            ai_system_prompts::product_name.eq(product_name.trim().to_lowercase()),
            ai_system_prompts::prompt.eq(prompt),
        );
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(ai_system_prompts::table)
            .values(data)
            .get_result::<AiSystemPrompt>(conn)
    }
}
