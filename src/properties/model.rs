use serde::Serialize;

use super::controllers::{FindPropertyQuery, PropertyWithAgent};
use crate::agents::AgentRole;
use crate::traits::Crud;
use crate::{
    db::DbPool,
    schema::{agents, properties},
};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, QueryResult,
    Queryable, RunQueryDsl,
};

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
    measurements: serde_json::Value,
    building_type: String,
    building_condition: BuildingCondition,
    building_furniture_capacity: Option<FurnitureCapacity>,
    building_certificate: String,
    specifications: serde_json::Value,
    facilities: serde_json::Value,
    is_deleted: bool,
}

impl Property {
    pub(super) fn find_many(
        pool: &DbPool,
        user_id: &Option<uuid::Uuid>,
        role: &Option<AgentRole>,
        query: &FindPropertyQuery,
    ) -> QueryResult<Vec<PropertyWithAgent>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let mut property_query = match role {
            Some(role) => match role {
                AgentRole::Admin => properties::table.into_boxed(),
                AgentRole::Agent => properties::table
                    .filter(
                        properties::user_id
                            .eq(user_id.unwrap())
                            .and(properties::is_deleted.eq(false)),
                    )
                    .into_boxed(),
            },
            None => properties::table
                .filter(
                    properties::is_deleted
                        .eq(false)
                        .and(properties::sold_status.eq(SoldStatus::Available)),
                )
                .into_boxed(),
        };

        match &query.s {
            Some(search_query) => match search_query.parse::<i32>() {
                Ok(id) => property_query = property_query.filter(properties::id.eq(id)),
                Err(_) => {
                    property_query = property_query.filter(
                        properties::title
                            .ilike(format!("%{}", search_query))
                            .or(properties::title.ilike(format!("%{}%", search_query)))
                            .or(properties::title.ilike(format!("{}%", search_query)))
                            .or(properties::street.ilike(format!("%{}", search_query)))
                            .or(properties::street.ilike(format!("%{}%", search_query)))
                            .or(properties::street.ilike(format!("{}%", search_query))),
                    )
                }
            },
            None => {}
        }

        property_query
            .inner_join(agents::table)
            .select((
                properties::all_columns,
                agents::fullname,
                agents::phone_number,
                agents::profile_picture_url,
            ))
            .order_by(properties::id.desc())
            .get_results::<(Property, String, String, Option<String>)>(conn)
    }

    pub(super) fn count_find_many_total(
        pool: &DbPool,
        user_id: &Option<uuid::Uuid>,
        role: &Option<AgentRole>,
    ) -> QueryResult<i64> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let property_query = match role {
            Some(role) => match role {
                AgentRole::Admin => properties::table.into_boxed(),
                AgentRole::Agent => properties::table
                    .filter(
                        properties::user_id
                            .eq(user_id.unwrap())
                            .and(properties::is_deleted.eq(false)),
                    )
                    .into_boxed(),
            },
            None => properties::table
                .filter(
                    properties::is_deleted
                        .eq(false)
                        .and(properties::sold_status.eq(SoldStatus::Available)),
                )
                .into_boxed(),
        };

        property_query.count().get_result(conn)
    }

    pub(super) fn find_one_by_id(pool: &DbPool, id: &i32) -> QueryResult<PropertyWithAgent> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        properties::table
            .filter(properties::id.eq(id))
            .inner_join(agents::table)
            .select((
                properties::all_columns,
                agents::fullname,
                agents::phone_number,
                agents::profile_picture_url,
            ))
            .get_result(conn)
    }
}

impl Crud for Property {
    type Output = Self;
    type SchemaTable = properties::table;
    type CreatePayload = CreatePropertySqlPayload;

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
}
