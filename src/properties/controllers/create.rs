use crate::middleware::Session;
use crate::properties::model::Property;
use crate::schema;
use crate::traits::Crud;
use crate::{
    db::DbPool,
    middleware::{AxumResponse, JsonResponse},
    properties::enumerates::{BuildingCondition, FurnitureCapacity, PurchaseStatus},
};
use axum::{
    extract::{Json, State},
    http::HeaderMap,
};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Images {
    path: String,
    english_label: String,
    indonesian_label: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct LandMeasurements {
    area: Option<i32>,
    width: Option<i32>,
    length: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct BuildingMeasurements {
    area: Option<i32>,
    width: Option<i32>,
    length: Option<i32>,
    height: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Specifications {
    bedrooms: Option<i32>,
    bathrooms: Option<i32>,
    garage: Option<i32>,
    carport: Option<i32>,
    electrical_power: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Facilities {
    value: String,
    english_label: String,
    indonesian_label: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CreatePropertyApiPayload {
    title: String,
    description: String,
    province: String,
    regency: String,
    street: String,
    gmap_iframe: Option<String>,
    price: i64,
    images: Vec<Images>,
    purchase_status: PurchaseStatus,
    land_measurements: LandMeasurements,
    building_type: String,
    building_condition: BuildingCondition,
    building_furniture_capacity: Option<FurnitureCapacity>,
    building_certificate: String,
    building_measurements: BuildingMeasurements,
    specifications: Specifications,
    facilities: Vec<Facilities>,
}

#[derive(Deserialize, Serialize, Insertable)]
#[diesel(table_name = schema::properties)]
pub(crate) struct CreatePropertySqlPayload {
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
    land_measurements: serde_json::Value,
    building_type: String,
    building_condition: BuildingCondition,
    building_furniture_capacity: Option<FurnitureCapacity>,
    building_certificate: String,
    building_measurements: serde_json::Value,
    specifications: serde_json::Value,
    facilities: serde_json::Value,
}

fn to_sql_payload(api_payload: &CreatePropertyApiPayload) -> CreatePropertySqlPayload {
    let purchase_status_slug = &api_payload.purchase_status.to_slug();
    let building_type_slug = &api_payload
        .building_type
        .trim()
        .replace(" ", "-")
        .to_lowercase();
    let regency_slug = &api_payload.regency.trim().replace(" ", "-").to_lowercase();
    let street_slug = &api_payload.street.trim().replace(" ", "-").to_lowercase();
    let site_path =
        format!("/{purchase_status_slug}/{building_type_slug}/{regency_slug}/{street_slug}");
    CreatePropertySqlPayload {
        site_path,
        title: api_payload.title.to_string(),
        description: api_payload.description.to_string(),
        province: api_payload.province.trim().to_lowercase(),
        regency: api_payload.regency.trim().to_lowercase(),
        street: api_payload.street.trim().to_lowercase(),
        gmap_iframe: api_payload.gmap_iframe.clone(),
        price: api_payload.price,
        images: serde_json::json!(&api_payload.images),
        purchase_status: api_payload.purchase_status.clone(),
        land_measurements: serde_json::json!(&api_payload.land_measurements),
        building_type: api_payload.building_type.to_string(),
        building_condition: api_payload.building_condition.clone(),
        building_furniture_capacity: api_payload.building_furniture_capacity.clone(),
        building_certificate: api_payload.building_certificate.to_string(),
        building_measurements: serde_json::json!(&api_payload.building_measurements),
        specifications: serde_json::json!(&api_payload.specifications),
        facilities: serde_json::json!(&api_payload.facilities),
    }
}

pub async fn create_property(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreatePropertyApiPayload>,
) -> AxumResponse<Property> {
    let user_id = Session::extract_session_user_id(&headers);
    let sql_payload = to_sql_payload(&payload);

    match Property::create(&pool, &user_id, &sql_payload) {
        Ok(property) => JsonResponse::send(201, Some(property), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}
