use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::repo)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Repo {
    pub uuid: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub branch: String,
    pub method: i32,
}
