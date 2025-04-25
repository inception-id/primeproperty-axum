// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "agent_role"))]
    pub struct AgentRole;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "building_condition"))]
    pub struct BuildingCondition;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "furniture_capacity"))]
    pub struct FurnitureCapacity;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "purchase_status"))]
    pub struct PurchaseStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "sold_status"))]
    pub struct SoldStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AgentRole;

    agents (id) {
        id -> Uuid,
        supertokens_user_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        fullname -> Varchar,
        email -> Varchar,
        phone_number -> Varchar,
        profile_picture_url -> Nullable<Varchar>,
        role -> AgentRole,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PurchaseStatus;
    use super::sql_types::SoldStatus;
    use super::sql_types::BuildingCondition;
    use super::sql_types::FurnitureCapacity;

    properties (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        site_path -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        #[max_length = 255]
        province -> Varchar,
        #[max_length = 255]
        regency -> Varchar,
        #[max_length = 255]
        street -> Varchar,
        gmap_iframe -> Nullable<Text>,
        price -> Int8,
        images -> Jsonb,
        purchase_status -> PurchaseStatus,
        sold_status -> SoldStatus,
        land_measurements -> Jsonb,
        #[max_length = 255]
        building_type -> Varchar,
        building_condition -> BuildingCondition,
        building_furniture_capacity -> Nullable<FurnitureCapacity>,
        #[max_length = 255]
        building_certificate -> Varchar,
        building_measurements -> Jsonb,
        specifications -> Jsonb,
        facilities -> Jsonb,
        is_deleted -> Bool,
    }
}

diesel::joinable!(properties -> agents (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    properties,
);
