use std::time::SystemTime;
use diesel::{QueryResult, Queryable, RunQueryDsl, ExpressionMethods};
use serde::{Deserialize, Serialize};
use crate::db::DbPool;
use crate::schema::users;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(crate) struct User {
    id: uuid::Uuid,
    supertokens_user_id: Option<String>,
    created_at: SystemTime,
    updated_at: SystemTime,
    email: String,
}

impl User {
    pub(super) fn create_user(
        pool: &DbPool,
        supertokens_user_id: &str,
        email: &str,
    ) -> QueryResult<User> {
        let data = (users::supertokens_user_id.eq(supertokens_user_id), users::email.eq(email));
        let conn = &mut pool.get().unwrap();
        diesel::insert_into(users::table).values(data).get_result::<User>(conn)
    }
}

