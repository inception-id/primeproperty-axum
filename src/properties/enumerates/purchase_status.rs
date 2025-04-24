use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, Deserialize, Serialize, Clone, FromSqlRow)]
#[diesel(sql_type = sql_types::PurchaseStatus)]
pub enum PurchaseStatus {
    ForSale,
    ForRent,
    ForSaleOrRent,
}

impl ToSql<sql_types::PurchaseStatus, Pg> for PurchaseStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            PurchaseStatus::ForSale => out.write_all(b"for_sale")?,
            PurchaseStatus::ForRent => out.write_all(b"for_rent")?,
            PurchaseStatus::ForSaleOrRent => out.write_all(b"for_sale_or_rent")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::PurchaseStatus, Pg> for PurchaseStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"for_sale" => Ok(PurchaseStatus::ForSale),
            b"for_rent" => Ok(PurchaseStatus::ForRent),
            b"for_sale_or_rent" => Ok(PurchaseStatus::ForSaleOrRent),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
