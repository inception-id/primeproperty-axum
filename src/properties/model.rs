use serde::Serialize;

use crate::traits::Crud;
use crate::{db::DbPool, schema::properties};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};

use super::{
    controllers::CreatePropertySqlPayload,
    enumerates::{BuildingCondition, FurnitureCapacity, PurchaseStatus, SoldStatus},
};

#[derive(Debug, Serialize, Queryable)]
pub(crate) struct Property {
    id: i32,
    user_id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    site_path: String,
    title: String,
    description: String,
    province: String,
    regency: String,
    street: String,
    gmap_iframe: Option<String>,
    price: i64,
    images: serde_json::Value,
    purchase_status: PurchaseStatus,
    sold_status: SoldStatus,
    land_measurements: serde_json::Value,
    building_type: String,
    building_condition: BuildingCondition,
    building_furniture_capacity: Option<FurnitureCapacity>,
    building_certificate: String,
    building_measurements: serde_json::Value,
    specifications: serde_json::Value,
    facilities: serde_json::Value,
}

impl Crud for Property {
    type Output = Self;
    type SchemaTable = properties::table;
    type CreatePayload = CreatePropertySqlPayload;
    type FindQueries = CreatePropertySqlPayload;

    fn create(
        pool: &DbPool,
        uuid: &uuid::Uuid,
        payload: &Self::CreatePayload,
    ) -> QueryResult<Self::Output> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::insert_into(properties::table)
            .values((properties::user_id.eq(uuid), payload))
            .get_result(conn)
    }

    fn find_many_by_user_id(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        queries: &Self::FindQueries,
    ) -> QueryResult<Vec<Self::Output>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        properties::table.get_results(conn)
    }

    fn count_find_many_by_user_id_total(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        queries: &Self::FindQueries,
    ) -> QueryResult<i64> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        properties::table
            .count()
            .filter(properties::user_id.eq(user_id))
            .get_result(conn)
    }
}
