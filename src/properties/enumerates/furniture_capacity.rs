use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, Deserialize, Serialize, Clone, FromSqlRow)]
#[diesel(sql_type = sql_types::FurnitureCapacity)]
pub enum FurnitureCapacity {
    Furnished,
    SemiFurnished,
    Unfurnished,
}

impl ToSql<sql_types::FurnitureCapacity, Pg> for FurnitureCapacity {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            FurnitureCapacity::Furnished => out.write_all(b"furnished")?,
            FurnitureCapacity::SemiFurnished => out.write_all(b"semi_furnished")?,
            FurnitureCapacity::Unfurnished => out.write_all(b"unfurnished")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::FurnitureCapacity, Pg> for FurnitureCapacity {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"furnished" => Ok(FurnitureCapacity::Furnished),
            b"semi_furnished" => Ok(FurnitureCapacity::SemiFurnished),
            b"unfurnished" => Ok(FurnitureCapacity::Unfurnished),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
