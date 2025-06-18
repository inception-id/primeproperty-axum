use super::controllers::{
    FindPropertyQuery, FindPropertySort, PropertyWithAgent, UpdateConfigurationsSqlPayload,
    AGENT_PAGE_SIZE, CLIENT_PAGE_SIZE,
};
use super::enumerates::{Currency, RentTime, SoldChannel};
use super::{
    controllers::CreateUpdatePropertySqlPayload,
    enumerates::{BuildingCondition, FurnitureCapacity, PurchaseStatus, SoldStatus},
};
use crate::agents::AgentRole;
use crate::traits::Crud;
use crate::{
    db::DbPool,
    schema::{agents, properties},
};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgJsonbExpressionMethods, PgTextExpressionMethods,
    QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

#[derive(Debug, Serialize, Queryable)]
pub struct Property {
    pub id: i32,
    pub user_id: uuid::Uuid,
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
    sold_channel: Option<SoldChannel>,
    configurations: serde_json::Value,
    currency: Currency,
    rent_time: Option<RentTime>,
}

impl Property {
    pub fn find_one_by_id(pool: &DbPool, id: &i32) -> QueryResult<PropertyWithAgent> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        properties::table
            .filter(properties::id.eq(id))
            .inner_join(agents::table)
            .select((
                properties::all_columns,
                agents::fullname,
                agents::phone_number,
                agents::profile_picture_url,
                agents::instagram,
            ))
            .get_result(conn)
    }

    pub(super) fn update(
        pool: &DbPool,
        id: &i32,
        payload: &CreateUpdatePropertySqlPayload,
    ) -> QueryResult<Property> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::update(properties::table.filter(properties::id.eq(id)))
            .set(payload)
            .get_result(conn)
    }

    pub(super) fn delete(pool: &DbPool, id: &i32, role: &AgentRole) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        match role {
            AgentRole::Admin => diesel::delete(properties::table)
                .filter(properties::id.eq(id))
                .get_result(conn),
            AgentRole::Agent => diesel::update(properties::table)
                .filter(properties::id.eq(id))
                .set(properties::is_deleted.eq(true))
                .get_result(conn),
        }
    }

    pub(super) fn update_configurations(
        pool: &DbPool,
        id: &i32,
        payload: &UpdateConfigurationsSqlPayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::update(properties::table.filter(properties::id.eq(id)))
            .set(payload)
            .get_result(conn)
    }

    pub fn find_distinct_site_paths(pool: &DbPool) -> QueryResult<Vec<String>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        properties::table
            .distinct_on(properties::site_path)
            .select(properties::site_path)
            .get_results(conn)
    }

    pub fn find_distinct_building_type_paths(
        pool: &DbPool,
    ) -> QueryResult<Vec<(PurchaseStatus, String)>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        properties::table
            .distinct_on((properties::purchase_status, properties::building_type))
            .select((properties::purchase_status, properties::building_type))
            .get_results(conn)
    }

    pub fn find_distinct_province_paths(
        pool: &DbPool,
    ) -> QueryResult<Vec<(PurchaseStatus, String, String)>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        properties::table
            .distinct_on((
                properties::purchase_status,
                properties::building_type,
                properties::province,
            ))
            .select((
                properties::purchase_status,
                properties::building_type,
                properties::province,
            ))
            .get_results(conn)
    }

    pub fn find_distinct_regency_paths(
        pool: &DbPool,
    ) -> QueryResult<Vec<(PurchaseStatus, String, String, String)>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        properties::table
            .distinct_on((
                properties::purchase_status,
                properties::building_type,
                properties::province,
                properties::regency,
            ))
            .select((
                properties::purchase_status,
                properties::building_type,
                properties::province,
                properties::regency,
            ))
            .get_results(conn)
    }
}

diesel::define_sql_function! {
    fn similarity(column: diesel::sql_types::Text, keyword: diesel::sql_types::Text) -> diesel::sql_types::Float
}

impl Crud for Property {
    type Output = Self;
    type SchemaTable = properties::table;
    type CreatePayload = CreateUpdatePropertySqlPayload;
    type FindManyOutput = PropertyWithAgent;
    type FindManyParam = FindPropertyQuery;

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

    fn find_many(
        pool: &DbPool,
        user_id: &Option<uuid::Uuid>,
        role: &Option<AgentRole>,
        query: &Self::FindManyParam,
    ) -> QueryResult<Vec<Self::FindManyOutput>> {
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
            None => match &query.s {
                Some(_) => properties::table
                    .distinct_on(properties::site_path)
                    .filter(
                        properties::is_deleted
                            .eq(false)
                            .and(properties::sold_status.eq(SoldStatus::Available)),
                    )
                    .into_boxed(),
                None => properties::table
                    .filter(
                        properties::is_deleted
                            .eq(false)
                            .and(properties::sold_status.eq(SoldStatus::Available)),
                    )
                    .into_boxed(),
            },
        };

        match &query.s {
            Some(search_query) => match search_query.parse::<i32>() {
                Ok(id) => property_query = property_query.filter(properties::id.eq(id)),
                Err(_) => {
                    property_query = property_query
                        .filter(similarity(properties::site_path, search_query).gt(0.1))
                }
            },
            None => {}
        }

        match &query.province {
            Some(province_query) => {
                property_query =
                    property_query.filter(properties::province.eq(province_query.to_lowercase()));
            }
            None => {}
        }

        match &query.regency {
            Some(regency_query) => {
                property_query =
                    property_query.filter(properties::regency.eq(regency_query.to_lowercase()));
            }
            None => {}
        }

        match &query.street {
            Some(street_query) => {
                property_query =
                    property_query.filter(properties::street.eq(street_query.to_lowercase()));
            }
            None => {}
        }

        match &query.is_popular {
            Some(is_popular) => {
                let filter_json = serde_json::json!({ "is_popular": is_popular});
                property_query =
                    property_query.filter(properties::configurations.contains(filter_json))
            }
            None => {}
        }

        match &query.sold_status {
            Some(sold_status) => {
                property_query = property_query.filter(properties::sold_status.eq(sold_status))
            }
            _ => {}
        }

        match &query.purchase_status {
            Some(purchase_status) => {
                property_query = property_query.filter(
                    properties::purchase_status
                        .eq(purchase_status)
                        .or(properties::purchase_status.eq(PurchaseStatus::ForSaleOrRent)),
                )
            }
            _ => {}
        }

        match &query.building_type {
            Some(build_type) => {
                property_query =
                    property_query.filter(properties::building_type.eq(build_type.to_lowercase()))
            }
            _ => {}
        }

        let page_size = match role {
            Some(_) => AGENT_PAGE_SIZE,
            None => CLIENT_PAGE_SIZE,
        };

        match &query.page {
            Some(page) => {
                let offset = (page - 1) * page_size;
                property_query = property_query.offset(offset).limit(page_size);
            }
            None => {
                property_query = property_query.limit(page_size);
            }
        };

        if let Some(sort) = &query.sort {
            match sort {
                FindPropertySort::LowestPrice => {
                    property_query = property_query.order_by(properties::price.asc())
                }
                FindPropertySort::HighestPrice => {
                    property_query = property_query.order_by(properties::price.desc())
                }
            }
        } else {
            match &query.s {
                Some(search_query) => match search_query.parse::<i32>() {
                    Ok(_) => property_query = property_query.order_by(properties::id.desc()),
                    Err(_) => {
                        property_query = property_query.order_by((
                            properties::site_path,
                            similarity(properties::site_path, search_query).desc(),
                        ))
                    }
                },
                None => property_query = property_query.order_by(properties::id.desc()),
            }
        }

        property_query
            .inner_join(agents::table)
            .select((
                properties::all_columns,
                agents::fullname,
                agents::phone_number,
                agents::profile_picture_url,
                agents::instagram,
            ))
            .get_results::<PropertyWithAgent>(conn)
    }

    fn count_find_many_rows(
        pool: &DbPool,
        user_id: &Option<uuid::Uuid>,
        role: &Option<AgentRole>,
        query: &Self::FindManyParam,
    ) -> QueryResult<i64> {
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

        match &query.province {
            Some(province_query) => {
                property_query =
                    property_query.filter(properties::province.eq(province_query.to_lowercase()));
            }
            None => {}
        }

        match &query.regency {
            Some(regency_query) => {
                property_query =
                    property_query.filter(properties::regency.eq(regency_query.to_lowercase()));
            }
            None => {}
        }

        match &query.is_popular {
            Some(is_popular) => {
                let filter_json = serde_json::json!({ "is_popular": is_popular});
                property_query =
                    property_query.filter(properties::configurations.contains(filter_json))
            }
            None => {}
        }

        match &query.sold_status {
            Some(sold_status) => {
                property_query = property_query.filter(properties::sold_status.eq(sold_status))
            }
            _ => {}
        }

        match &query.purchase_status {
            Some(purchase_status) => {
                property_query = property_query.filter(
                    properties::purchase_status
                        .eq(purchase_status)
                        .or(properties::purchase_status.eq(PurchaseStatus::ForSaleOrRent)),
                )
            }
            _ => {}
        }

        match &query.building_type {
            Some(build_type) => {
                property_query =
                    property_query.filter(properties::building_type.eq(build_type.to_lowercase()))
            }
            _ => {}
        }

        property_query.count().get_result(conn)
    }
}
