use diesel::prelude::*;
use diesel::PgConnection;
use chrono::{DateTime, Utc};

use crate::models::item::{NewItem, Item, ItemResponse, ItemFilter, ItemVec};
use crate::models::reference::{NewItemRef, ItemRef};
use crate::cal::occurs_between;

// this is a big function. How can I break it up into something smaller?
pub fn get_items(
  conn: &PgConnection,
  filters: ItemFilter
) -> Result<ItemVec, diesel::result::Error> {
  use crate::schema::items::dsl::*;
  use crate::schema::item_references;

  let mut response = ItemVec {
    items: vec![],
    refs: vec![]
  };

  let mut item_limit = match filters.limit {
    Some(limit) => limit,
    None => 2000
  };

  // Start building the DB query
  let mut query = items.limit(item_limit).into_boxed(); // .into_boxed() is required to add constraints to the query and make it dynamic

  match filters.deleted {
    Some(value) => query = query.filter(deleted.eq(value)),
    None => query = query.filter(deleted.eq(false))
  };

  match filters.journal {
    Some(value) => query = query.filter(journal.eq(value)),
    None => ()
  };

  match filters.cal {
    Some(value) => query = query.filter(cal.eq(value)),
    None => ()
  };

  match filters.todo {
    Some(value) => query = query.filter(todo.eq(value)),
    None => ()
  };

  // match filters.creator_id { // add this once we add users
  //   Some(creator_id) => query = query.filter(user_id.eq(creator_id)),
  //   None => ()
  // }

  match filters.item_id {
    Some(search_id) => query = query.filter(id.eq(search_id)),
    None => println!("no id")
  };

  match filters.parent_id {
    Some(search_parent) => query = query.filter(parent_id.eq(search_parent)),
    None => ()
  };

  match (filters.created_before, filters.created_after) {
    (Some(start), Some(end)) => {
      let mut real_start = start;
      let mut real_end = end;
      if (start > end) {
        // honestly let's just throw a 401 here
        real_start = end;
        real_end = start;
      }
      let real_start = real_start.naive_utc();
      let real_end = real_end.naive_utc();
      query = query
        .filter(created_at.ge(real_start))
        .filter(created_at.le(real_end));
    }
    (Some(start), None) => {
      let start = start.naive_utc();
      query = query.filter(created_at.ge(start));
    }
    (None, Some(end)) => {
      let end = end.naive_utc();
      query = query.filter(created_at.le(end));
    }
    _ => ()
  }

  match (filters.updated_before, filters.updated_after) {
    (Some(start), Some(end)) => {
      let mut real_start = start;
      let mut real_end = end;
      if (start > end) {
        // honestly let's just throw a 401 here
        real_start = end;
        real_end = start;
      }
      let real_start = real_start.naive_utc();
      let real_end = real_end.naive_utc();
      query = query
        .filter(updated_at.ge(real_start))
        .filter(updated_at.le(real_end));
    }
    (Some(start), None) => {
      let start = start.naive_utc();
      query = query.filter(updated_at.ge(start));
    }
    (None, Some(end)) => {
      let end = end.naive_utc();
      query = query.filter(updated_at.le(end));
    }
    _ => ()
  }

  // Load the items from the DB
  let selected_items = query.load::<Item>(conn)?;

  // post-load modifications (Date filtering)

  match (filters.occurs_after, filters.occurs_before) {
    (Some(start), Some(end)) => {
      response.items = occurs_between(selected_items, start, end);
    }
    // (Some(start), None) => (), //
    // (None, Some(start)) => (),
    _ => response.items = selected_items
  };

  let mut selected_ids: Vec<i32> = vec![];
  for i in 0..response.items.len() {
    selected_ids.push(response.items[i].id);
  }

  match filters.with_related {
    Some(true) => {
      // println!("with_related is disabled, fix this :)");
      let refs = item_references::table
        .filter(item_references::origin_id.eq_any(&selected_ids))
        .or_filter(item_references::child_id.eq_any(&selected_ids));
      // println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&refs));
      let refs = refs.load::<ItemRef>(conn)?;

      response.refs = refs;
    }
    _ => ()
  }

  Ok(response)
}

pub fn get_item_by_id(
  item_id: i32,
  conn: &PgConnection
) -> Result<Option<Item>, diesel::result::Error> {
  use crate::schema::items::dsl::*;

  let item_query = items.filter(id.eq(item_id));
  println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&item_query));
  let item = item_query.first::<Item>(conn).optional()?;

  Ok(item)
}

pub fn upsert_item(
  new_item: NewItem,
  references: Vec<NewItemRef>,
  conn: &PgConnection
) -> Result<Item, diesel::result::Error> {
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

pub fn insert_references(
  new_id: i32,
  mut references: Vec<NewItemRef>,
  conn: &PgConnection
) -> Result<(), diesel::result::Error> {
  use crate::schema::item_references::dsl::*;

  for i in 0..references.len() {
    match references[i] {
      NewItemRef {
        origin_id: None,
        child_id: Some(_)
      } => references[i].origin_id = Some(new_id),
      NewItemRef {
        origin_id: Some(_),
        child_id: None
      } => references[i].child_id = Some(new_id),
      _ => ()
    }
  }

  let _q = diesel::insert_into(item_references)
    .values(references)
    .execute(conn)?;
  Ok(())
}

pub fn delete_item_by_id(item_id: i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {
  use crate::schema::items::dsl::*;

  let _del_query = diesel::delete(items.filter(id.eq(item_id))).execute(conn)?;

  Ok(())
}

pub fn get_parents_by_child_id(
  item_id: i32,
  conn: &PgConnection
) -> Result<Vec<Item>, diesel::result::Error> {
  use crate::schema::item_references;

  let users = item_references::table
    .filter(item_references::child_id.eq(item_id))
    .select(item_references::origin_id);
  use crate::schema::items::dsl::*;
  let data: Vec<Item> = items.filter(id.eq_any(users)).load(conn).unwrap();

  Ok(data)
}

pub fn get_children_by_parent_id(
  item_id: i32,
  conn: &PgConnection
) -> Result<Vec<Item>, diesel::result::Error> {
  use crate::schema::item_references;

  let users = item_references::table
    .filter(item_references::origin_id.eq(item_id))
    .select(item_references::child_id);
  use crate::schema::items::dsl::*;
  let data: Vec<Item> = items.filter(id.eq_any(users)).load(conn).unwrap();

  Ok(data)
}

pub fn get_references_by_id(
  item_id: i32,
  conn: &PgConnection
) -> Result<Option<ItemResponse>, diesel::result::Error> {
  let item = get_item_by_id(item_id, conn)?;

  if let Some(_) = item {
    println!("we got one");
    let parents = get_parents_by_child_id(item_id, conn)?;
    let children = get_children_by_parent_id(item_id, conn)?;
    let response = ItemResponse {
      item_id: item_id,
      item: item,
      parents: parents,
      children: children
    };
    Ok(Some(response))
  } else {
    println!("we didn't find anything :(");
    Ok(None)
  }
}
