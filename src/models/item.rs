use chrono::{NaiveDate, NaiveDateTime, Utc, TimeZone, DateTime};
type UtcDateTime = DateTime<Utc>;
use crate::{Repetition, TaskLike};
use crate::old_task::{Mod, Mods};
use std::str::FromStr;
use std::fmt;
use std::fmt::Display;
use serde::{Serialize, Deserialize};

use crate::schema::{items};
// use crate::models::reference;

#[derive(Queryable, Identifiable, Serialize)] // may want to use AsChangeset here, does funky things with optionals though.
#[table_name = "items"]
pub struct Item {
  pub id: i32,
  pub created_at: NaiveDateTime, // not sure if these support timezone. Could be an issue. DB might need to run a to/from UTC function before sending/after recieving objects from the client.
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
  pub cal: bool
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
    match self.start_d {
      Some(dt) => Some(dt.date()),
      None => None
    }
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
  pub cal: Option<bool>
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
      cal: None
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

// For retrieving from API calls
#[derive(Deserialize, Debug)]
pub struct NewItemTz {
  pub id: Option<i32>,
  pub start_d: Option<DateTime<Utc>>,
  pub end_d: Option<DateTime<Utc>>,
  pub repeats: Option<String>,
  pub title: String,
  pub note: Option<String>,
  pub marked_done: bool,
  pub deleted: bool,
  pub journal: bool,
  pub todo: bool,
  pub cal: bool
}

// I don't understand how this type signature works, it feels like I'm casting the type backwards from the argument? is that allowed? Compiler says it's ok so ¯\_(ツ)_/¯
// I think this works because DateTime _requires_ a tz, but it doesn't matter what it is, the functions all work agnostic of the tz
pub fn opt_utc_to_naive<Tz: TimeZone>(dt_tz: Option<DateTime<Tz>>) -> Option<NaiveDateTime> {
  match dt_tz {
    Some(date) => Some(date.naive_utc()),
    None => None
  }
}

impl From<NewItemTz> for NewItem {
  fn from(old_item: NewItemTz) -> Self {
    Self {
      id: old_item.id,
      start_d: opt_utc_to_naive(old_item.start_d),
      end_d: opt_utc_to_naive(old_item.end_d),
      repeats: old_item.repeats,
      title: old_item.title,
      note: old_item.note,
      marked_done: Some(old_item.marked_done),
      deleted: Some(old_item.deleted),
      journal: Some(old_item.journal),
      todo: Some(old_item.todo),
      cal: Some(old_item.cal)
    }
  }
}

#[derive(Serialize)]
pub struct ItemResponse {
  pub item_id: i32,
  pub item: Option<Item>,
  pub parents: Vec<Item>,
  pub children: Vec<Item>
}

#[derive(Serialize)]
pub struct ItemVec {
  pub items: Vec<Item>,
  pub refs: Vec<crate::models::reference::ItemRef>
}

// ######################## ITEM FILTER QUERY TYPES
#[derive(Deserialize)]
pub struct ItemFilter {
  pub item_id: Option<i32>,    // done
  pub creator_id: Option<i32>, // no users yet
  pub title_filter: Option<String>,
  pub body_filter: Option<String>,
  pub deleted: Option<bool>,  // done
  pub parent_id: Option<i32>, // done
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
      title_filter: None,
      body_filter: None,
      deleted: None,
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
