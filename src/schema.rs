// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "agent_role"))]
    pub struct AgentRole;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AgentRole;

    agents (id) {
        id -> Uuid,
        supertokens_user_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Varchar,
        phone_number -> Varchar,
        role -> AgentRole,
    }
}
