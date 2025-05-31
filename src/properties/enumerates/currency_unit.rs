use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, Deserialize, Serialize, Clone, FromSqlRow)]
#[diesel(sql_type = sql_types::CurrencyUnit)]
pub enum Currency {
    Idr,
    Usd,
}

impl ToSql<sql_types::CurrencyUnit, Pg> for Currency {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Currency::Idr => out.write_all(b"IDR")?,
            Currency::Usd => out.write_all(b"USD")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::CurrencyUnit, Pg> for Currency {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"IDR" => Ok(Currency::Idr),
            b"USD" => Ok(Currency::Usd),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
