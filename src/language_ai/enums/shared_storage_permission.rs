use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, Deserialize, Serialize, Clone)]
#[diesel(sql_type = sql_types::SharedStoragePermission)]
pub enum SharedStoragePermission {
    View,
    Edit,
}

impl ToSql<sql_types::SharedStoragePermission, Pg> for SharedStoragePermission {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            SharedStoragePermission::View => out.write_all(b"view")?,
            SharedStoragePermission::Edit => out.write_all(b"edit")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::SharedStoragePermission, Pg> for SharedStoragePermission {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"view" => Ok(SharedStoragePermission::View),
            b"edit" => Ok(SharedStoragePermission::Edit),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
