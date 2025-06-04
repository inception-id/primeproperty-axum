use crate::{
    properties::enumerates::{PurchaseStatus, SoldStatus},
    traits::Crud,
};
use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
};
use serde::Deserialize;

use crate::{
    agents::Agent,
    db::DbPool,
    middleware::{AxumResponse, JsonFindResponse, JsonResponse, Session},
    properties::model::Property,
};

#[derive(Deserialize)]
pub enum FindPropertySort {
    LowestPrice,
    HighestPrice,
}

pub(crate) const AGENT_PAGE_SIZE: i64 = 20;
pub(crate) const CLIENT_PAGE_SIZE: i64 = 20;
#[derive(Deserialize)]
pub struct FindPropertyQuery {
    pub s: Option<String>,
    pub province: Option<String>,
    pub regency: Option<String>,
    pub street: Option<String>,
    pub page: Option<i64>,
    pub is_popular: Option<bool>,
    pub sold_status: Option<SoldStatus>,
    pub purchase_status: Option<PurchaseStatus>,
    pub building_type: Option<String>,
    pub sort: Option<FindPropertySort>,
}

pub(crate) type PropertyWithAgent = (Property, String, String, Option<String>, Option<String>);

pub async fn find_many_properties(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Query(query): Query<FindPropertyQuery>,
) -> AxumResponse<JsonFindResponse<Vec<PropertyWithAgent>>> {
    let header_user_id = headers.get("x-user-id");
    let (user_id, role) = match header_user_id {
        Some(_) => {
            let user_id = Session::extract_session_user_id(&headers);
            match Agent::find_by_user_id(&pool, &user_id) {
                Ok(agent) => (Some(user_id), Some(agent.role)),
                _ => {
                    return JsonResponse::send(403, None, None);
                }
            }
        }
        None => (None, None),
    };

    let property_with_agent = match Property::find_many(&pool, &user_id, &role, &query) {
        Ok(property_with_agent) => property_with_agent,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    let page_size = match role {
        Some(_) => AGENT_PAGE_SIZE,
        None => CLIENT_PAGE_SIZE,
    };

    let total_property_count = match Property::count_find_many_rows(&pool, &user_id, &role, &query)
    {
        Ok(property_with_agent_count) => property_with_agent_count,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    JsonResponse::send(
        200,
        Some(JsonFindResponse {
            data: property_with_agent,
            total_pages: (total_property_count / page_size) + 1,
            total_data: total_property_count,
        }),
        None,
    )
}

pub async fn find_one_by_id(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> AxumResponse<PropertyWithAgent> {
    match Property::find_one_by_id(&pool, &id) {
        Ok(property) => JsonResponse::send(200, Some(property), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub async fn find_site_paths(State(pool): State<DbPool>) -> AxumResponse<Vec<String>> {
    let mut site_paths = vec![
        format!("/{}", PurchaseStatus::ForSale.to_slug()),
        format!("/{}", PurchaseStatus::ForRent.to_slug()),
    ];
    if let Ok(building_types) = Property::find_distinct_building_type_paths(&pool) {
        for (purchase_status, b_type) in building_types {
            let path = format!(
                "/{}/{}",
                purchase_status.to_slug(),
                b_type.replace(" ", "-")
            );
            site_paths.push(path);
        }
    }
    if let Ok(provinces) = Property::find_distinct_province_paths(&pool) {
        for (purchase_status, b_type, province) in provinces {
            let path = format!(
                "/{}/{}/{}",
                purchase_status.to_slug(),
                b_type.replace(" ", "-"),
                province.replace(" ", "-")
            );
            site_paths.push(path);
        }
    }

    if let Ok(regencies) = Property::find_distinct_regency_paths(&pool) {
        for (purchase_status, b_type, province, regency) in regencies {
            let path = format!(
                "/{}/{}/{}/{}",
                purchase_status.to_slug(),
                b_type.replace(" ", "-"),
                province.replace(" ", "-"),
                regency.replace(" ", "-")
            );
            site_paths.push(path);
        }
    }
    if let Ok(distinct_site_paths) = Property::find_distinct_site_paths(&pool) {
        for path in distinct_site_paths {
            site_paths.push(path);
        }
    }
    JsonResponse::send(200, Some(site_paths), None)
}
