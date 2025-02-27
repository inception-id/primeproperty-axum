use crate::language_ai::SharedStoragePermission;
use crate::schema::sql_types;
use chrono::NaiveDateTime;
use diesel::sql_types::{Integer, Nullable, Text, Timestamp, Uuid, VarChar};
use diesel::QueryableByName;
use serde::Serialize;

#[derive(QueryableByName, Debug, Serialize)]
pub struct SharedTranslationStorageJoinTranslationStorage {
    #[diesel(sql_type=Integer)]
    shared_storage_id: i32,
    #[diesel(sql_type=Integer)]
    storage_id: i32,
    #[diesel(sql_type=Uuid)]
    owner_id: uuid::Uuid,
    #[diesel(sql_type=VarChar)]
    owner_email: String,
    #[diesel(sql_type= sql_types::SharedStoragePermission)]
    permission: SharedStoragePermission,
    #[diesel(sql_type=Timestamp)]
    created_at: NaiveDateTime,
    #[diesel(sql_type=Timestamp)]
    updated_at: NaiveDateTime,
    #[diesel(sql_type=VarChar)]
    content_language: String,
    #[diesel(sql_type=VarChar)]
    target_language: String,
    #[diesel(sql_type=Nullable<VarChar>)]
    title: Option<String>,
    #[diesel(sql_type=Text)]
    content: String,
    #[diesel(sql_type=Text)]
    updated_completion: String,
}
