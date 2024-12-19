use crate::db::DbPool;
use crate::schema::users;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(super) struct User {
    id: uuid::Uuid,
    supertokens_user_id: Option<String>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    email: String,
}

impl User {
    pub(super) fn create_user(
        pool: &DbPool,
        supertokens_user_id: &str,
        email: &str,
    ) -> QueryResult<User> {
        let data = (
            users::supertokens_user_id.eq(supertokens_user_id),
            users::email.eq(email),
        );
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(users::table)
            .values(data)
            .get_result::<User>(conn)
    }

    pub(super) fn find_user_by_email(pool: &DbPool, email: &str) -> QueryResult<User> {
        let conn = &mut pool.get().unwrap();
        users::table.filter(users::email.eq(email)).get_result(conn)
    }
}
