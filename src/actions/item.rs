use diesel::{prelude::*};
use diesel::PgConnection;

use crate::models::item::{NewItem, Item, ItemResponse, ItemFilter, RefsItem};
use crate::models::reference::{NewItemRef, ItemRef};
use crate::models::tags::{Tag, NewItemTag};
use crate::cal::occurs_between;
use crate::TaskLike;
use crate::util::child_order_range_error::ChildOrderRangeError;
use crate::util::diesel_error::DieselError;
use crate::util::errors::TodoError;

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
    None => ()
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

/*
 * test for ^
 * init db if necessary
 * create connection
 *
 * seed an item (hold the id)
 * request the item by id
 * verify it's the same item
 *
 * request an item we know doesn't exist (id 9999999999)
 * verify no result
 *
 * request an item with a negative id (-1)
 * verify no result
 */
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

/*
 * test for upsert_item
 *
 * init test db if needed
 * get connection
 *
 * make item { title: "upsert_test_{$testID}" }, save id
 * verify insert
 * run function again with an upsert { id: inserted_id, title: "upsert_test_{$testID} (UPDATED)"}
 * verify update, same ID
 *
 * negative cases:
 *  * missing field
 *  * bad dates
 *  * bad parent_id
*/
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

  // REMOVE REFS
  // println!("{:?}", references);
  // if references.len() > 0 {
  //   match set_references_for_parent(references, inserted_item.id.unwrap(), conn) {
  //     Ok(refs) => inserted_item.references = refs,
  //     Err(e) => eprintln!("an error occurred! {}", e)
  //   }
  // }

  if tags.len() > 0 {
    match insert_tags(tags, inserted_item.get_id(), conn) {
      Ok(inserted) => inserted_item.tags = inserted,
      Err(e) => eprintln!("an error occurred! {}", e)
    }
  }

  Ok(inserted_item)
}

// this error just needs to contain both diesel errors and user input errors
/**
 * Returns the IDs of the two items whose `child_order`s got swapped
 */
pub fn shift_up(item_id: i32, conn: &PgConnection) -> Result<(i32, i32), Box<dyn TodoError>> {
  use crate::schema::items::dsl::*;

  let item1 = items
    .filter(id.eq(item_id))
    .first::<Item>(conn)
    .map_err(DieselError::from)?;

  if item1.child_order < 1 {
    return Err(Box::new(ChildOrderRangeError {}));
  }

  let trans = conn
    .build_transaction()
    .run::<(i32, i32), diesel::result::Error, _>(|| {
      let item2_id = items
        .filter(child_order.eq(item1.child_order - 1))
        .filter(parent_id.eq(item1.parent_id))
        .select(id)
        .first::<i32>(conn)?;

      // calls to diesel inside the transaction don't need to map_err because the transaction itself has a map_err on it
      diesel::update(items)
        .set(child_order.eq(-1))
        .filter(id.eq(item2_id))
        .execute(conn)?;
      diesel::update(items)
        .set(child_order.eq(item1.child_order - 1))
        .filter(id.eq(item1.id))
        .execute(conn)?;
      diesel::update(items)
        .set(child_order.eq(item1.child_order))
        .filter(id.eq(item2_id))
        .execute(conn)?;
      Ok((item_id, item2_id))
    })
    .map_err(DieselError::from)?; // this does some funky stuff with type inference and box<dyn TodoError> but it's FINE it's FINE
  Ok(trans)
}

/**
 * Returns the IDs of the two items whose `child_order`s got swapped
 */
pub fn shift_down(item_id: i32, conn: &PgConnection) -> Result<(i32, i32), Box<dyn TodoError>> {
  use crate::schema::items::dsl::*;

  let item1 = items
    .filter(id.eq(item_id))
    .first::<Item>(conn)
    .map_err(DieselError::from)?;

  let item2 = items
    .filter(parent_id.eq(item1.parent_id))
    .order_by(child_order.desc())
    .first::<Item>(conn)
    .map_err(DieselError::from)?;
  if item1.child_order > item2.child_order {
    return Err(Box::new(ChildOrderRangeError {}));
  }
  let trans = conn
    .build_transaction()
    .run::<(i32, i32), diesel::result::Error, _>(|| {
      let item1 = items.filter(id.eq(item_id)).first::<Item>(conn)?;

      diesel::update(items)
        .filter(id.eq(item2.id))
        .set(child_order.eq(-1))
        .execute(conn)?;
      diesel::update(items)
        .filter(id.eq(item1.id))
        .set(child_order.eq(item2.child_order))
        .execute(conn)?;
      diesel::update(items)
        .filter(id.eq(item2.id))
        .set(child_order.eq(item1.child_order))
        .execute(conn)?;
      Ok((item1.id, item2.id))
    })
    .map_err(DieselError::from)?;
  Ok(trans)
}

/**
 * indent_item
 * takes an `item_id` for item a, finds its immediate neighbor b, and assigns b's item_id to a->parent_id
 */
pub fn indent_item(item_a_id: i32, conn: &PgConnection) -> Result<Item, diesel::result::Error> {
  use crate::schema::items::dsl::*;
  use diesel::expression::count::count;
  let item_a = items.filter(id.eq(item_a_id)).first::<Item>(conn)?;

  // get the first item with child_order less than a with the same parent.
  let item_b = items
    .filter(parent_id.eq(item_a.parent_id))
    .filter(child_order.lt(item_a.child_order))
    .order_by(child_order.desc())
    .first::<Item>(conn)?;

  // get the # of children that B has
  let count: i64 = items
    .select(count(id))
    .filter(parent_id.eq(item_b.id))
    .first(conn)?;

  diesel::update(items)
    .filter(id.eq(item_a.id))
    .set(child_order.eq(count as i32))
    .get_result::<Item>(conn)
}

/**
 * outdent_item
 * takes an `item_id` for item a, finds its parent b, then makes item a siblings of item b, adjusting all the child_order values somehow
 */
pub fn outdent_item(item_a_id: i32, conn: &PgConnection) -> Result<Item, diesel::result::Error> {
  use crate::schema::items::dsl::*;

  let item_a = items.filter(id.eq(item_a_id)).first::<Item>(conn)?;

  match item_a.parent_id {
    None => return Err(diesel::result::Error::NotFound),
    Some(item_a_parent_id) => {
      // fails if no parent, is that ok
      let item_b = items.filter(id.eq(item_a_parent_id)).first::<Item>(conn)?;

      conn
        .build_transaction()
        .run::<Item, diesel::result::Error, _>(|| {
          // update items with parent_id.eq(item_b.parent_id) so that child_order > item_b.child_order += 1
          // make room for item A
          diesel::update(items)
            .filter(parent_id.eq(item_b.parent_id)) // OK if item_b doesn't have a parent :)
            .filter(child_order.gt(item_b.child_order))
            .set(child_order.eq(child_order + 1))
            .execute(conn)?;

          // update children of item B such that any with child_order > old a.child_order get decremented
          diesel::update(items)
            .filter(parent_id.eq(item_a_parent_id))
            .filter(child_order.lt(item_a.child_order))
            .set(child_order.eq(child_order - 1))
            .execute(conn)?;

          // update item A with new parent and child order values
          diesel::update(items)
            .filter(id.eq(item_a.id))
            .set((
              parent_id.eq(item_b.parent_id),
              child_order.eq(item_b.child_order)
            ))
            .get_result::<Item>(conn) // this returns

          // the client needs to put A in the right place (new parent ID takes care of this, new child_order takes care of this)
          // the client doesn't care about updating B's children, they can have gaps, that's OK?
          // except if we cut a hole in the children of B and then give B a new child, then the items might have collisions?
          // if you add children to an item, the client needs to ask for the updated list of children for that item. this
          // when I'm adding an item as a child, I have to make sure that I'm not getting the length, but incrementing the value of the last child OR i have to normalize them
        })
    }
  }
}

/*
 * test for insert_tags
 * (regular setup. ifn db init testdb, get connection)
 *
 * make item { title: "insert_tags_{$testID}", tags: ["bad"] }, save the id.
 * insert_tags(saved_id, ["static_tag", "${testID}"])
 * verify that "bad" is no longer present
 * verify that the static tag and testid tag are present
 *
 * insert_tags(bad_id, ["junk_tag"])
 * verify that insert_tags catches, wraps, and throws an error for bad id
 * verify that "junk_tag" isn't saved to DB.
 *
 * are there unallowed tags? What other limits does this function have?
 */
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

/* REMOVE REFS
 * test for set_references_for_parent
 *
 * expected behavior:
 *  passed parent, list of child ids.
 *  insert [ { parent_id: $id, child_id: $child }... ]
 *  children items get parent_id set in Items table.
 *  old references (child_id is unique) get modified with new parent id.
 *  new references get added
 *
 * failure points:
 *  bad ids
 *  ids not updating
 *  duplicate refs with [parent: x, child, a], [parent: y, child: a]
 */
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

/* REMOVE REFS
 * given a reference `NewItemRef`, validates that it contains both parts before inserting it, and updating the appropriate child item
 * Returns Result<Item, diesel::result::Error> containing either the updated child item
 * Returns Err(NotFound) if either part is missing
*/
// this needs to deal with `child_order` in some way, either by taking option<i32> or w/e
pub fn insert_reference(
  reference: NewItemRef,
  new_order: i32,
  conn: &PgConnection
) -> Result<Item, diesel::result::Error> {
  match (reference.origin_id, reference.child_id) {
    (Some(origin_value), Some(child_value)) => {
      {
        use crate::schema::item_references::dsl::*;

        let _query = diesel::insert_into(item_references)
          .values(reference)
          .on_conflict(child_id)
          .do_update()
          .set(origin_id.eq(origin_value))
          .execute(conn)?;
      }
      {
        use crate::schema::items::dsl::*;
        use diesel::expression::count::count;
        let count: i64 = items
          .select(count(id))
          .filter(parent_id.eq(origin_value))
          .first(conn)?;
        let count = count as i32;
        // if new_order < 0, set to 0
        // if new_order > count, set to count
        let new_order = match new_order {
          o if o < 0 => 0,
          o if o > count => count,
          _ => new_order
        };

        let updated_child = diesel::update(items.filter(id.eq(child_value)))
          .set((parent_id.eq(origin_value), child_order.eq(new_order)))
          .get_result::<Item>(conn)?;
        Ok(updated_child)
      }
    }
    _ => Err(diesel::result::Error::NotFound)
  }
}

/*
 * given a parent_id, references should get deleted
 * children in the deleted references should get their parent_id cleared.
*/
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

pub fn delete_child_ref(
  target_id: i32,
  conn: &PgConnection
) -> Result<Item, diesel::result::Error> {
  let mut old_parent_id: i32 = 0;
  // {
  //   use crate::schema::item_references::dsl::*;
  //   old_parent_id = item_references.select(origin_id).filter(child_id.eq(target_id)).first(conn)?;
  //   let _del_query =
  //     diesel::delete(item_references.filter(child_id.eq(target_id))).execute(conn)?;
  // }
  {
    use crate::schema::items::dsl::*;
    // get the old child_order location
    let order_number: i32 = items
      .select(child_order)
      .filter(id.eq(old_parent_id))
      .first(conn)?;
    // using the child_order, increment every sibling below the current
    diesel::update(
      items
        .filter(parent_id.is_null())
        .filter(child_order.ge(order_number))
    ) // since we're deleting the reference (parent_id -> no parent id) filter using is_null
    .set(child_order.eq(child_order + 1))
    .execute(conn)?;
    // now update the child_id on the item in question
    let item = diesel::update(items.filter(id.eq(target_id)))
      .set(parent_id.eq(None::<i32>))
      .get_result::<Item>(conn)?;
    Ok(item)
  }
}

/*
 * item should get deleted
 * failure cases:
 *  wrong id
 *  references existing in other tables
 */
pub fn delete_item_by_id(item_id: i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {
  use crate::schema::items::dsl::*;

  let _del_query = diesel::delete(items.filter(id.eq(item_id))).execute(conn)?;

  Ok(())
}

// REMOVE REFS
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

//REMOVE REFS
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

// do I have a use case for this?
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
