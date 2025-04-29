use crate::db::DbPool;
use axum::routing::{get, post, put};
use axum::Router;

mod create_update;
mod find;

pub(crate) use create_update::CreateUpdatePropertySqlPayload;
pub(crate) use find::{FindPropertyQuery, PropertyWithAgent, PAGE_SIZE};

pub fn property_routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(create_update::create_property))
        .route("/{id}", put(create_update::update_property))
        .route("/", get(find::find_many_properties))
        .route("/{id}", get(find::find_one_by_id))
}
