use chrono::{NaiveDate, NaiveDateTime, Utc, DateTime};
// use diesel::update;
type UtcDateTime = DateTime<Utc>;
use crate::{Repetition, TDError, TaskLike};
use crate::old_task::{Mod, Mods};
use std::str::FromStr;
use std::fmt;
use std::fmt::Display;
use serde::{Serialize, Deserialize};

use crate::schema::{items};
// use crate::models::reference;

/* TODO: This file has too many silly little models in it.
  Either:
    * Remove some of these models
    * Move them into somewhere else and only use them where they're needed.

    Examples:
    - Item (exactly matches the table)
    - RefsItem (Item but optional and with reference fields attached. Must be destructured to store)
      - This should be changed so that all fields are optional. Error handling should be explicit, and the actix-web type checking isn't verbose enough
    - NewItem (I think this was meant to fill the space of the RefsItem struct)
    - ItemResponse (Server response item, contains references and item ID and stuff. This should be removed and usages replaced with refsitem)
    - ItemFilter (Query struct, this should be replaced with RefsItem.)
*/

#[derive(Queryable, Identifiable, Serialize)] // may want to use AsChangeset here, does funky things with optionals though.
#[table_name = "items"]
pub struct Item {
  pub id: i32,
  pub created_at: NaiveDateTime, // utc: make sure to translate to UTC before saving.
  pub updated_at: NaiveDateTime,
  pub start_d: Option<NaiveDateTime>,
  pub end_d: Option<NaiveDateTime>,
  pub repeats: String,
  pub title: String,
  pub note: Option<String>,
  pub marked_done: bool, // I'm not sure if we need this, should probably be another table ngl
  pub deleted: bool,
  pub parent_id: Option<i32>,
  pub journal: bool,
  pub todo: bool,
  pub cal: bool,
  pub user_id: Option<i32>
}

impl Item {
  // use this to get access to the notes
  fn note_or_empty(&self) -> String {
    match &self.note {
      Some(note) => note.to_string(),
      None => String::from("n/a")
    }
  }
}

impl TaskLike for Item {
  // TODO: this is the most used part of tasklike for Item. This could be broken out into a separate Trait
  fn get_id(&self) -> i32 {
    self.id
  }

  fn formatted_date(&self) -> String {
    // this is wonky, needs to parse into a timezoned date, since they're in UTC
    match (self.start_d, self.end_d) {
      (Some(start), Some(end)) => format!("{} - {}", start, end),
      (Some(start), None) => format!("{}", start),
      _ => String::new()
    }
  }

  fn get_start(&self) -> Option<NaiveDate> {
    self.start_d.map(|dt| dt.date())
  }

  fn get_rep(&self) -> Repetition {
    Repetition::from_str(&self.repeats.to_string()).unwrap_or_else(|_| Repetition::Never)
  }

  fn is_finished(&self) -> bool {
    self.marked_done
  }

  // TODO: This isn't finished yet
  fn get_last_completed(&self) -> Option<&NaiveDate> {
    None
  }

  fn to_string(&self) -> String {
    String::from(format!("{}", self))
  }
}

impl Display for Item {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{id} - {title}\n{range}\n{rep}\nNotes: {note}\nFinished: {finished}",
      id = self.id,
      title = self.title,
      range = self.formatted_date(),
      rep = self.repeats,
      note = self.note_or_empty(),
      finished = self.marked_done
    )
  }
}

// by default the dates are parsed according to RFC 3339 https://tools.ietf.org/html/rfc3339#page-10
// for creating and inserting
#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "items"]
pub struct NewItem {
  pub id: Option<i32>,
  pub start_d: Option<NaiveDateTime>,
  pub end_d: Option<NaiveDateTime>,
  pub repeats: Option<String>,
  pub title: String,
  pub note: Option<String>,
  pub marked_done: Option<bool>, // I'm not sure if we need this
  pub deleted: Option<bool>,
  pub journal: Option<bool>,
  pub todo: Option<bool>,
  pub cal: Option<bool>,
  pub user_id: Option<i32>
}

impl NewItem {
  pub fn new() -> Self {
    NewItem {
      id: None,
      start_d: None,
      end_d: None,
      repeats: None,
      title: String::new(),
      note: None,
      marked_done: None,
      deleted: None,
      journal: None,
      todo: None,
      cal: None,
      user_id: None
    }
  }
}

impl From<Mods> for NewItem {
  fn from(mods: Mods) -> Self {
    let mut update = NewItem::new();

    for modification in mods.data {
      match modification {
        Mod::Start(date) => update.start_d = date,
        Mod::End(date) => update.end_d = date,
        Mod::Rep(rep) => update.repeats = Some(rep),
        Mod::Title(new_title) => update.title = new_title,
        Mod::Note(new_note) => update.note = Some(new_note),
        Mod::Cal(value) => update.cal = Some(value),
        Mod::Todo(value) => update.todo = Some(value),
        Mod::Journal(value) => update.journal = Some(value)
      }
    }

    update
  }
}

#[derive(Serialize)]
pub struct ItemResponse {
  pub item_id: i32,
  pub item: Option<Item>,
  pub parents: Vec<Item>,
  pub children: Vec<Item>
}

#[derive(Serialize, Deserialize)]
pub struct RefsItem {
  pub id: Option<i32>,
  pub created_at: Option<NaiveDateTime>, // This should be a DateTime, we can turn it into UTC when we need to.
  pub updated_at: Option<NaiveDateTime>,
  pub start_d: Option<NaiveDateTime>,
  pub end_d: Option<NaiveDateTime>,
  pub repeats: Option<String>,
  pub title: Option<String>,
  pub note: Option<String>,
  pub marked_done: Option<bool>,
  pub deleted: Option<bool>,
  pub parent_id: Option<i32>,
  pub journal: Option<bool>,
  pub todo: Option<bool>,
  pub cal: Option<bool>,
  pub user_id: Option<i32>,

  // these parts make it more complicated. We send/recieve this with the client and must
  // translate to Item before talking to the DB
  pub references: Vec<crate::models::reference::ItemRef>,
  pub tags: Vec<crate::models::tags::Tag>
}

impl std::convert::TryFrom<RefsItem> for NewItem {
  type Error = TDError;
  fn try_from(item: RefsItem) -> Result<Self, TDError> {
    // required fields: title
    if let Some(title) = item.title {
      Ok(NewItem {
        id: item.id,
        title: title,
        start_d: item.start_d,
        end_d: item.end_d,
        repeats: item.repeats,
        note: item.note,
        marked_done: item.marked_done,
        deleted: item.deleted,
        // parent_id: item.parent_id,
        journal: item.journal,
        todo: item.todo,
        cal: item.cal,
        user_id: item.user_id
      })
    } else {
      Err(TDError::ParseError(String::from(
        "Error constructing NewItem from RefsItem. Missing field: title"
      )))
    }
  }
}

impl From<Item> for RefsItem {
  fn from(item: Item) -> Self {
    return Self {
      id: Some(item.id),
      created_at: Some(item.created_at),
      updated_at: Some(item.updated_at),
      start_d: item.start_d,
      end_d: item.end_d,
      repeats: Some(item.repeats),
      title: Some(item.title),
      note: match &item.note {
        Some(s) => Some(String::from(s)),
        _ => None
      },
      marked_done: Some(item.marked_done),
      deleted: Some(item.deleted),
      parent_id: item.parent_id,
      journal: Some(item.journal),
      todo: Some(item.todo),
      cal: Some(item.cal),
      user_id: item.user_id,

      references: vec![],
      tags: vec![]
    };
  }
}

impl RefsItem {
  fn note_or_empty(&self) -> String {
    match &self.note {
      Some(note) => note.to_string(),
      None => String::from("n/a")
    }
  }
}

// This TaskLike impl is used only for display. Display functions should be moved out into a separate Trait
impl TaskLike for RefsItem {
  fn get_id(&self) -> i32 {
    match self.id {
      Some(i) => i,
      None => -1
    }
  }

  fn formatted_date(&self) -> String {
    // this is wonky, needs to parse into a timezoned date, since they're in UTC
    match (self.start_d, self.end_d) {
      (Some(start), Some(end)) => format!("{} - {}", start, end),
      (Some(start), None) => format!("{}", start),
      _ => String::new()
    }
  }

  fn get_start(&self) -> Option<NaiveDate> {
    match self.start_d {
      Some(dt) => Some(dt.date()),
      None => None
    }
  }

  fn get_rep(&self) -> Repetition {
    match &self.repeats {
      Some(r) => Repetition::from_str(r).unwrap_or_else(|_| Repetition::Never),
      None => Repetition::Never
    }
  }

  fn is_finished(&self) -> bool {
    self.marked_done.unwrap_or_else(|| false)
  }

  // TODO: This isn't finished yet
  fn get_last_completed(&self) -> Option<&NaiveDate> {
    None
  }

  // unused
  fn to_string(&self) -> String {
    String::from(format!("{}", self))
  }
}

// I think this isn't used
impl Display for RefsItem {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let title = String::from(match &self.title {
      Some(t) => t,
      _ => {
        eprintln!("Called print on a title-less item! How did you get here?");
        "n/a"
      }
    });
    write!(
      f,
      "{id} - {title}\n{range}\n{rep}\nNotes: {note}\nFinished: {finished}",
      id = self.get_id(),
      title = title,
      range = self.formatted_date(),
      rep = self.get_rep(),
      note = self.note_or_empty(),
      finished = self.marked_done.unwrap_or_else(|| false)
    )
  }
}

// ######################## ITEM FILTER QUERY TYPES
#[derive(Deserialize)]
pub struct ItemFilter {
  pub item_id: Option<i32>,    // done
  pub creator_id: Option<i32>, // no users yet
  pub title: Option<String>,
  pub note: Option<String>,
  pub deleted: Option<bool>,  // done
  pub parent_id: Option<i32>, // done
  pub tags: Option<String>,   // works
  pub limit: Option<i64>,     // done
  // type
  pub journal: Option<bool>,
  pub todo: Option<bool>,
  pub cal: Option<bool>,
  // structural
  pub with_related: Option<bool>, // done
  // dates
  pub occurs_after: Option<UtcDateTime>, // 2012-03-29T10:05:45-06:00
  pub occurs_before: Option<UtcDateTime>,
  pub created_before: Option<UtcDateTime>,
  pub created_after: Option<UtcDateTime>,
  pub updated_before: Option<UtcDateTime>,
  pub updated_after: Option<UtcDateTime>,
  pub repeats: Option<String>
}

impl ItemFilter {
  pub fn new() -> Self {
    ItemFilter {
      item_id: None,
      creator_id: None,
      title: None,
      note: None,
      deleted: None,
      tags: None,
      parent_id: None,
      limit: None,
      journal: None,
      todo: None,
      cal: None,
      with_related: None,
      occurs_after: None,
      occurs_before: None,
      created_before: None,
      created_after: None,
      updated_before: None,
      updated_after: None,
      repeats: None
    }
  }
}
