use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, Deserialize, Serialize, Clone, FromSqlRow)]
#[diesel(sql_type = sql_types::BuildingCondition)]
pub enum BuildingCondition {
    New,
    Good,
    Renovated,
    RenovationRequired,
    Old,
}

impl ToSql<sql_types::BuildingCondition, Pg> for BuildingCondition {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            BuildingCondition::New => out.write_all(b"new")?,
            BuildingCondition::Good => out.write_all(b"good")?,
            BuildingCondition::Renovated => out.write_all(b"renovated")?,
            BuildingCondition::RenovationRequired => out.write_all(b"renovation_required")?,
            BuildingCondition::Old => out.write_all(b"old")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::BuildingCondition, Pg> for BuildingCondition {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"new" => Ok(BuildingCondition::New),
            b"good" => Ok(BuildingCondition::Good),
            b"renovated" => Ok(BuildingCondition::Renovated),
            b"renovation_required" => Ok(BuildingCondition::RenovationRequired),
            b"old" => Ok(BuildingCondition::Old),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
