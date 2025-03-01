use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, Deserialize, Serialize, Clone)]
#[diesel(sql_type = sql_types::TarsChatMessagesRole)]
pub enum TarsChatMessagesRole {
    Developer,
    System,
    User,
    Assistant,
    Tool,
}

impl ToSql<sql_types::TarsChatMessagesRole, Pg> for TarsChatMessagesRole {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TarsChatMessagesRole::Developer => out.write_all(b"developer")?,
            TarsChatMessagesRole::System => out.write_all(b"system")?,
            TarsChatMessagesRole::User => out.write_all(b"user")?,
            TarsChatMessagesRole::Assistant => out.write_all(b"assistant")?,
            TarsChatMessagesRole::Tool => out.write_all(b"tool")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::TarsChatMessagesRole, Pg> for TarsChatMessagesRole {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"developer" => Ok(TarsChatMessagesRole::Developer),
            b"system" => Ok(TarsChatMessagesRole::System),
            b"user" => Ok(TarsChatMessagesRole::User),
            b"assistant" => Ok(TarsChatMessagesRole::Assistant),
            b"tool" => Ok(TarsChatMessagesRole::Tool),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
