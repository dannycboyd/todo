use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

use crate::schema::item_references;


// #[derive(Queryable)]
// pub struct Ref {
//   created_at: NaiveDateTime,
//   parent_task: i32,
//   parent_note: i32,
//   child_task: i32,
//   child_note: i32
// }

#[derive(Queryable, Serialize, Debug)]
pub struct ItemRef {
  pub id: i32,
  pub created_at: NaiveDateTime,
  pub origin_id: i32,
  pub child_id: i32
}

#[derive(Insertable, Deserialize, Debug, AsChangeset)]
#[table_name = "item_references"]
pub struct NewItemRef {
  pub origin_id: Option<i32>,
  pub child_id: Option<i32>
}