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
        agent_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        address -> Text,
        gmap_iframe -> Nullable<Text>,
        price -> Int4,
        images -> Jsonb,
        purchase_status -> PurchaseStatus,
        sold_status -> SoldStatus,
        land_area -> Nullable<Int4>,
        land_width -> Nullable<Int4>,
        land_length -> Nullable<Int4>,
        #[max_length = 255]
        building_type -> Varchar,
        building_condition -> BuildingCondition,
        building_furniture_capacity -> Nullable<FurnitureCapacity>,
        #[max_length = 255]
        building_certificate -> Varchar,
        building_levels -> Int4,
        building_area -> Nullable<Int4>,
        building_width -> Nullable<Int4>,
        building_length -> Nullable<Int4>,
        building_height -> Nullable<Int4>,
        bedrooms_count -> Nullable<Int4>,
        bathrooms_count -> Nullable<Int4>,
        garage_capacity -> Nullable<Int4>,
        carport_capacity -> Nullable<Int4>,
        electrical_power -> Nullable<Int4>,
        facilities -> Jsonb,
    }
}

diesel::joinable!(properties -> agents (agent_id));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    properties,
);
