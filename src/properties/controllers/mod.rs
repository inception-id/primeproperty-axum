use crate::db::DbPool;
use axum::routing::{get, post};
use axum::Router;

mod create;
mod find;

pub(crate) use create::CreatePropertySqlPayload;
pub(crate) use find::{FindPropertyQuery, PropertyWithAgent};

pub fn property_routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(create::create_property))
        .route("/", get(find::find_many_properties))
        .route("/{id}", get(find::find_one_by_id))
}
