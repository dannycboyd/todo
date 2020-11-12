use chrono::NaiveDateTime;
use crate::schema::users;

#[derive(Queryable, Identifiable)] // may want to use AsChangeset here, does funky things with optionals though.
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  firstname: String,
  lastname: String,
  prefix: Option<String>,
  note: Option<String>,
  deleted: bool,
  hash: String,
  salt: String
}
