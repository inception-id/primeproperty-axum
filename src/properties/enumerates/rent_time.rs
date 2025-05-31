use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, Deserialize, Serialize, Clone, FromSqlRow)]
#[diesel(sql_type = sql_types::RentTimeUnit)]
pub enum RentTime {
    Monthly,
    Yearly,
}

impl ToSql<sql_types::RentTimeUnit, Pg> for RentTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            RentTime::Monthly => out.write_all(b"monthly")?,
            RentTime::Yearly => out.write_all(b"yearly")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::RentTimeUnit, Pg> for RentTime {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"monthly" => Ok(RentTime::Monthly),
            b"yearly" => Ok(RentTime::Yearly),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
