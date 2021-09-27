use diesel::{prelude::*};
use diesel::PgConnection;

use crate::models::item::{NewItem, Item, ItemResponse, ItemFilter, RefsItem};
use crate::models::reference::{NewItemRef, ItemRef};
use crate::models::tags::{Tag, NewItemTag};
use crate::cal::occurs_between;
use crate::TaskLike;

// this is a big function. How can I break it up into something smaller?
pub fn get_items(
  conn: &PgConnection,
  filters: ItemFilter
) -> Result<Vec<RefsItem>, diesel::result::Error> {
  use crate::schema::items::dsl::*;
  use crate::schema::item_references;
  use crate::schema::tags as tags_table;

  let item_limit = match filters.limit {
    Some(limit) => limit,
    None => 2000
  };

  // Start building the DB query
  let mut query = items.order(id.asc()).limit(item_limit).into_boxed(); // .into_boxed() is required to add constraints to the query and make it dynamic

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
      if start > end {
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
      if start > end {
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

  match filters.tags {
    Some(filter_tags) => {
      let split_tags: Vec<&str> = filter_tags.split(",").collect();
      println!("{:?}", split_tags);
      let allowed_ids = tags_table::table
        .filter(tags_table::tag.eq_any(&split_tags))
        .distinct()
        .select(tags_table::item_id)
        .load::<i32>(conn)?;
      query = query.filter(id.eq_any(allowed_ids));
    }

    None => ()
  }

  // Load the items from the DB
  let selected_items = query.load::<Item>(conn)?;

  // post-load modifications (Date filtering)

  let mut selected_ids: Vec<i32> = vec![];
  let mut response: Vec<RefsItem> = match (filters.occurs_after, filters.occurs_before) {
    (Some(start), Some(end)) => occurs_between(selected_items, start, end),
    // (Some(start), None) => (), //
    // (None, Some(start)) => (),
    _ => selected_items
  }
  .into_iter()
  .map(|i| {
    selected_ids.push(i.id);
    RefsItem::from(i)
  })
  .collect();

  let tags_vec = tags_table::table
    .filter(tags_table::item_id.eq_any(&selected_ids))
    .load::<Tag>(conn)?;

  for i in tags_vec {
    let mut index = 0;
    while index < response.len() && response[index].get_id() != i.item_id {
      index += 1;
    }
    response[index].tags.push(i);
  }

  match filters.with_related {
    Some(true) => {
      // println!("with_related is disabled, fix this :)");
      let refs = item_references::table
        .filter(item_references::origin_id.eq_any(&selected_ids))
        .or_filter(item_references::child_id.eq_any(&selected_ids));
      // println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&refs));
      let refs = refs.load::<ItemRef>(conn)?;

      for i in refs {
        let mut index = 0;
        while index < response.len() && response[index].get_id() != i.origin_id {
          index += 1;
        }
        if index < response.len() {
          let origin_id = i.origin_id;

          // update parent_id on the child
          // find child
          match response
            .iter_mut()
            .find(|child| child.get_id() == i.child_id)
          {
            Some(item) => {
              item.parent_id = Some(origin_id);
            }
            None => {}
          }
          response[index].references.push(i);
        } else {
          println!(
            "Error! no item found with id {}, but reference has ids {}, {}",
            i.origin_id, i.origin_id, i.child_id
          );
        }
      }
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
  references: Vec<i32>,
  tags: Vec<String>, // I guess for now we delete + reinsert
  conn: &PgConnection
) -> Result<RefsItem, diesel::result::Error> {
  use crate::schema::items::dsl::*;
  // println!("{:?}", new_item);

  let our_query = diesel::insert_into(items)
    .values(&new_item)
    .on_conflict(id)
    .do_update()
    .set(&new_item);
  // println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&our_query));
  let inserted_item = our_query.get_result::<Item>(conn)?;
  let mut inserted_item = RefsItem::from(inserted_item);

  println!("{:?}", references);
  if references.len() > 0 {
    match set_references_for_parent(references, inserted_item.id.unwrap(), conn) {
      Ok(refs) => inserted_item.references = refs,
      Err(e) => eprintln!("an error occurred! {}", e)
    }
  }

  if tags.len() > 0 {
    match insert_tags(tags, inserted_item.get_id(), conn) {
      Ok(inserted) => inserted_item.tags = inserted,
      Err(e) => eprintln!("an error occurred! {}", e)
    }
  }

  Ok(inserted_item)
}

pub fn insert_tags(
  tags_values: Vec<String>,
  parent_id: i32,
  conn: &PgConnection
) -> Result<Vec<Tag>, diesel::result::Error> {
  use crate::schema::tags::dsl::*;

  // let _del_query = diesel::delete(tags.filter(tag.ne_all(item_id))).execute(conn)?;
  let _del = diesel::delete(tags.filter(item_id.eq(parent_id))).execute(conn)?;
  let tags_values: Vec<NewItemTag> = tags_values
    .into_iter()
    .map(|t| NewItemTag {
      tag: t,
      item_id: Some(parent_id)
    })
    .collect();

  diesel::insert_into(tags)
    .values(tags_values)
    .get_results::<Tag>(conn)
}

pub fn set_references_for_parent(
  children: Vec<i32>,
  parent: i32,
  conn: &PgConnection
) -> Result<Vec<ItemRef>, diesel::result::Error> {
  let result = conn.transaction::<_, diesel::result::Error, _>(|| {
    let child_ids = children.iter();
    let mut updated_rows: usize = 0;

    let refs = child_ids.map(|child| {
      let new_item = NewItemRef {
        origin_id: Some(parent),
        child_id: Some(*child)
      };

      {
        // update parent_id on child items
        use crate::schema::items::dsl::*;
        let _q = diesel::update(items.filter(id.eq(child)))
          .set(parent_id.eq(parent))
          .execute(conn);
        if let Ok(count) = _q {
          updated_rows += count;
        }
      };
      let result = {
        // insert or update refs
        use crate::schema::item_references::dsl::*;
        diesel::insert_into(item_references)
          .values(&new_item)
          .on_conflict(child_id)
          .do_update()
          .set(origin_id.eq(parent))
          .get_result::<ItemRef>(conn)
      };
      result
    });
    let foo: Result<Vec<ItemRef>, diesel::result::Error> = refs.collect();
    foo
  });
  result
}

// maybe better as (parent: i32, references: array<i32>)
// when will I want to shotgun a bunch of unrelated references in? probably never.
pub fn insert_references(
  // new_id: i32,
  references: Vec<NewItemRef>,
  conn: &PgConnection
) -> Result<Vec<ItemRef>, diesel::result::Error> {
  use crate::schema::item_references::dsl::*;

  let result = conn.transaction::<_, diesel::result::Error, _>(|| {
    let need_updates = references.iter();

    let mut updated: usize = 0;
    // update children[].parent_id
    for reference in need_updates {
      use crate::schema::items::dsl::*;
      let child_id_to_set = reference.child_id.unwrap();
      let origin_id_to_set = reference.origin_id.unwrap();
      let _q = diesel::update(items.filter(id.eq(child_id_to_set)))
        .set(parent_id.eq(origin_id_to_set))
        .execute(conn);
      if let Ok(count) = _q {
        updated += count;
      }
    }
    let updated_ids: Vec<i32> = references
      .iter()
      .map(|reference| reference.origin_id.unwrap())
      .collect();

    let _q = diesel::insert_into(item_references)
      .values(references)
      .execute(conn)?;

    let updated_references = item_references
      .filter(origin_id.eq_any(updated_ids))
      .get_results::<ItemRef>(conn);

    println!("inserted references, updated {} children", updated);
    Ok(updated_references)
  })?;

  result
}

pub fn delete_references(
  _references: Vec<NewItemRef>,
  _conn: &PgConnection
) -> Result<(), diesel::result::Error> {
  // use crate::schema::item_references::dsl::*;

  // let _q = diesel::delete(item_reference.filter()) // how does this filter work?
  //   .execute(conn)
  println!("This function is unimplemented! :D");
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
