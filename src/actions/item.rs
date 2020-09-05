use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::item::{NewItem, Item};
use crate::models::reference::NewItemRef;

pub fn get_item_by_id(item_id: i32, conn: &PgConnection) -> Result<Option<Item>, diesel::result::Error> {
  use crate::schema::items::dsl::*;

  let item_query = items.filter(id.eq(item_id));
  println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&item_query));
  let item = item_query
    .first::<Item>(conn)
    .optional()?;

  Ok(item)
}

pub fn upsert_item(new_item: NewItem, references: Vec<NewItemRef>, conn: &PgConnection) -> Result<Item, diesel::result::Error> {
  use crate::schema::items::dsl::*;
  // println!("{:?}", new_item);

  let our_query = diesel::insert_into(items)
    .values(&new_item)
    .on_conflict(id)
    .do_update()
    .set(&new_item);
  // println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&our_query));
  let inserted_item = our_query.get_result::<Item>(conn)?;

  println!("{:?}", references);
  if references.len() > 0 {
    insert_references(inserted_item.id, references, conn)?;
  }

  Ok(inserted_item)
}

pub fn insert_references(new_id: i32, mut references: Vec<NewItemRef>, conn: &PgConnection) -> Result<(), diesel::result::Error> {
  use crate::schema::item_references::dsl::*;

  for i in 0..references.len() {
    match references[i] {
      NewItemRef { origin_id: None, child_id: Some(_) } => references[i].origin_id = Some(new_id),
      NewItemRef { origin_id: Some(_), child_id: None } => references[i].child_id = Some(new_id),
      _ => ()
    }
  }

  let _q = diesel::insert_into(item_references)
    .values(references)
    .execute(conn)?;
  Ok(())
}