use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::schema::users;

#[derive(Queryable, Identifiable)] // may want to use AsChangeset here, does funky things with optionals though.
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub firstname: String,
  pub lastname: String,
  pub prefix: Option<String>,
  pub note: Option<String>,
  pub deleted: bool,
  pub pwd_hash: String,
  pub pwd_salt: Vec<u8>
}

#[derive(Deserialize)]
pub struct NewUserRequest {
  pub user: NewUser,
  pub password: String
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
  pub firstname: Option<String>,
  pub lastname: Option<String>,
  pub prefix: Option<String>,
  pub note: Option<String>,
  pub pwd_hash: Option<String>,
  pub pwd_salt: Option<Vec<u8>>
}

#[derive(Deserialize)]
pub struct LoginRequest {
  pub id: i32,
  pub password: String
}
