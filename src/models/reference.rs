use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

use crate::schema::item_references;

/**
  After thinking about it for a while, i need to codify what origin/child means, especially when creating references.
  origin is the parent, upstream, comes first in the mental model/map
  When creating references, however, they are usually backwards, going from the child to the parent
*/
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct ItemRef {
  pub created_at: NaiveDateTime,
  pub origin_id: i32,
  pub child_id: i32,
  pub updated_at: NaiveDateTime
}

#[derive(Insertable, Deserialize, Debug, AsChangeset, Clone)]
#[table_name = "item_references"]
pub struct NewItemRef {
  pub origin_id: Option<i32>,
  pub child_id: Option<i32>
}
