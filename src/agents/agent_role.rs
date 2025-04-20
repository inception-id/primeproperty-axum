use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, Deserialize, Serialize, Clone, FromSqlRow)]
#[diesel(sql_type = sql_types::AgentRole)]
pub enum AgentRole {
    Admin,
    Agent,
}

impl ToSql<sql_types::AgentRole, Pg> for AgentRole {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            AgentRole::Admin => out.write_all(b"admin")?,
            AgentRole::Agent => out.write_all(b"agent")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::AgentRole, Pg> for AgentRole {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"admin" => Ok(AgentRole::Admin),
            b"agent" => Ok(AgentRole::Agent),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
