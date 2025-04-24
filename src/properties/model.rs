use diesel::prelude::Queryable;
use serde::Deserialize;

use super::enumerates::{BuildingCondition, FurnitureCapacity, PurchaseStatus, SoldStatus};

#[derive(Debug, Deserialize, Queryable)]
pub struct Property {
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
    price: i32,
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
