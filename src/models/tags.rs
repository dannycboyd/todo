use chrono::NaiveDateTime;
// use diesel::update;
use serde::{Deserialize, Serialize};

use crate::schema::tags;
#[derive(Queryable, Identifiable, Serialize, Deserialize, Clone)] // may want to use AsChangeset here, does funky things with optionals though.
#[table_name = "tags"]
pub struct Tag {
  pub id: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub item_id: i32,
  pub tag: String
}

#[derive(Deserialize, Insertable)]
#[table_name = "tags"]
pub struct NewItemTag {
  pub item_id: Option<i32>,
  pub tag: String
}
