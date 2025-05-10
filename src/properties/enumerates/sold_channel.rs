use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, Deserialize, Serialize, Clone, FromSqlRow)]
#[diesel(sql_type = sql_types::SoldChannel)]
pub enum SoldChannel {
    Web,
    R123,
    Socmed,
    Banner,
    Others,
}

impl ToSql<sql_types::SoldChannel, Pg> for SoldChannel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            SoldChannel::Web => out.write_all(b"web")?,
            SoldChannel::R123 => out.write_all(b"r123")?,
            SoldChannel::Socmed => out.write_all(b"socmed")?,
            SoldChannel::Banner => out.write_all(b"banner")?,
            SoldChannel::Others => out.write_all(b"others")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::SoldChannel, Pg> for SoldChannel {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"web" => Ok(SoldChannel::Web),
            b"r123" => Ok(SoldChannel::R123),
            b"socmed" => Ok(SoldChannel::Socmed),
            b"banner" => Ok(SoldChannel::Banner),
            b"others" => Ok(SoldChannel::Others),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
