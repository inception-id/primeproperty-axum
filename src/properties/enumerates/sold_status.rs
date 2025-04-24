use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, Deserialize, Serialize, Clone, FromSqlRow)]
#[diesel(sql_type = sql_types::SoldStatus)]
pub enum SoldStatus {
    Available,
    Sold,
}

impl ToSql<sql_types::SoldStatus, Pg> for SoldStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            SoldStatus::Available => out.write_all(b"available")?,
            SoldStatus::Sold => out.write_all(b"sold")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::SoldStatus, Pg> for SoldStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"available" => Ok(SoldStatus::Available),
            b"sold" => Ok(SoldStatus::Sold),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
