use axum::Router;

use crate::db::DbPool;
use axum::routing::post;
mod create;

pub(crate) use create::CreatePropertySqlPayload;

pub fn property_routes() -> Router<DbPool> {
    Router::new().route("/", post(create::create_property))
}
