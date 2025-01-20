use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, Deserialize, Serialize, Clone)]
#[diesel(sql_type = sql_types::PaymentStatus)]
pub enum PaymentStatus {
    Success,
    Pending,
    Fail,
}

impl ToSql<sql_types::PaymentStatus, Pg> for PaymentStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            PaymentStatus::Success => out.write_all(b"success")?,
            PaymentStatus::Pending => out.write_all(b"pending")?,
            PaymentStatus::Fail => out.write_all(b"fail")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::PaymentStatus, Pg> for PaymentStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"success" => Ok(PaymentStatus::Success),
            b"pending" => Ok(PaymentStatus::Pending),
            b"fail" => Ok(PaymentStatus::Fail),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
