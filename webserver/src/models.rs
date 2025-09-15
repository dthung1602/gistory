use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::{AsExpression, FromSqlRow, deserialize, serialize};
use serde::Serialize;

#[derive(Debug, Serialize, AsExpression, FromSqlRow, Clone, Copy, PartialEq, Eq)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum RepoStatus {
    New,
    InProgress,
    Done,
    Error,
}

impl deserialize::FromSql<Text, diesel::sqlite::Sqlite> for RepoStatus
where
    String: deserialize::FromSql<Text, diesel::sqlite::Sqlite>,
{
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let s = <String as deserialize::FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.to_ascii_lowercase().as_str() {
            "new" => Ok(RepoStatus::New),
            "in_progress" | "inprogress" => Ok(RepoStatus::InProgress),
            "done" => Ok(RepoStatus::Done),
            "error" => Ok(RepoStatus::Error),
            other => Err(format!("Unrecognized RepoStatus string: {}", other).into()),
        }
    }
}

impl serialize::ToSql<Text, diesel::sqlite::Sqlite> for RepoStatus
where
    String: serialize::ToSql<Text, diesel::sqlite::Sqlite>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> serialize::Result {
        let s: &str = match self {
            RepoStatus::New => "new",
            RepoStatus::InProgress => "in_progress",
            RepoStatus::Done => "done",
            RepoStatus::Error => "error",
        };
        out.set_value(s.to_string());
        Ok(serialize::IsNull::No)
    }
}

#[derive(Serialize, Queryable, Selectable, Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::repo)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Repo {
    pub uuid: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub branch: String,
    pub method: i32,
    pub status: RepoStatus,
}
