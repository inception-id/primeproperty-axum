use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, Deserialize, Serialize, Clone)]
#[diesel(sql_type = sql_types::StorageVisibility)]
pub enum StorageVisibility {
    Public,
    Private,
}

impl ToSql<sql_types::StorageVisibility, Pg> for StorageVisibility {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            StorageVisibility::Public => out.write_all(b"public")?,
            StorageVisibility::Private => out.write_all(b"private")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::StorageVisibility, Pg> for StorageVisibility {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"public" => Ok(StorageVisibility::Public),
            b"private" => Ok(StorageVisibility::Private),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
