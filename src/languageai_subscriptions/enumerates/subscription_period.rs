use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use crate::schema::sql_types;
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, Deserialize, Serialize, Clone)]
#[diesel(sql_type = sql_types::SubscriptionPeriod)]
pub enum SubscriptionPeriod {
    OneYear,
    ThreeMonths,
    OneMonth,
}

impl SubscriptionPeriod {
    pub fn to_month_count(self) -> i32 {
        match self { 
            SubscriptionPeriod::OneYear => 12,
            SubscriptionPeriod::ThreeMonths => 3,
            SubscriptionPeriod::OneMonth => 3,
        }
    }
}

impl ToSql<sql_types::SubscriptionPeriod, Pg> for SubscriptionPeriod {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            SubscriptionPeriod::OneYear => out.write_all(b"one_year")?,
            SubscriptionPeriod::ThreeMonths => out.write_all(b"three_months")?,
            SubscriptionPeriod::OneMonth => out.write_all(b"one_month")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::SubscriptionPeriod, Pg> for SubscriptionPeriod {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"one_year" => Ok(SubscriptionPeriod::OneYear),
            b"three_months" => Ok(SubscriptionPeriod::ThreeMonths),
            b"one_month" => Ok(SubscriptionPeriod::OneMonth),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
