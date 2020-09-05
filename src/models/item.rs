use chrono::{NaiveDate, NaiveDateTime, Utc, TimeZone, DateTime};
use crate::{Repetition, TaskLike};
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
  pub marked_done: bool, // I'm not sure if we need this
  pub deleted: bool,
  pub parent_id: Option<i32>,
  pub journal: bool,
  pub todo: bool,
  pub cal: bool,
}

impl Item {
  // use this to get access to the notes
  fn note_or_empty(&self) -> String {
    match &self.note {
      Some(note) => { note.to_string() },
      None => String::from("n/a")
    }
  }

}

impl TaskLike for Item {
  fn formatted_date(&self) -> String { // this is wonky
    match (self.start_d, self.end_d) {
      (Some(start), Some(end)) => {
        // print range
        format!("{} - {}", start, end)
      },
      (Some(start), None) => {
        // print date
        format!("{}", start)
      },
      _ => {
        // no date
        String::new()
      }
    }
  }

  fn get_start(&self) -> Option<NaiveDate> {
    match self.start_d {
      Some(dt) => Some(dt.date()),
      None => None
    }
  }

  fn get_rep(&self) -> Repetition {
    Repetition::from_str(&self.repeats.to_string())
      .unwrap_or_else(|_| Repetition::Never)
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
    write!(f, "{id} - {title}\n{range}\n{rep}\nNotes: {note}\nFinished: {finished}",
        id=self.id,
        title=self.title,
        range=self.formatted_date(),
        rep=self.repeats,
        note=self.note_or_empty(),
        finished=self.marked_done)
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

/*
Problem: doing updates requires either a hand-rolled query or a AsChangeset object with optional fields
problem: this struct has no id, can't be used for updates
problem: I want a chance to translate dates from tz into utc

I need a layer between the API and the internals
  incoming
    - dateTimeTz -> naiveDateTime
    - POST: determine insert or update
  outgoing
    - naiveDateTime -> tz? only if requests have a timezone on them, probably don't do this, and expect requesters to know their own timezone
  

*/

// #[derive(Insertable)]
// #[table_name = "items"]
// pub struct ChangeItem {
//   pub created_at: Option<NaiveDateTime>,
//   pub updated_at: Option<NaiveDateTime>,
//   pub start_d: Option<NaiveDateTime>,
//   pub end_d: Option<NaiveDateTime>,
//   pub repeats: Option<String>,
//   pub title: Option<String>,
//   pub note: Option<String>,
//   pub marked_done: bool, // I'm not sure if we need this
//   pub deleted: bool,
//   pub journal: bool,
//   pub todo: bool,
//   pub cal: bool,
// }

// impl NewItem {
//   pub fn new(form: actix_web::web::Json<NewItem>) -> Self {
//     Self {
//       start_d: None,
//       end_d: None,
//       repeats: String::from("n"),
//       title: String::new(),
//       note: None,
//       marked_done: false,
//       deleted: false,
//       journal: false,
//       todo: false,
//       cal: false
//     }
//   }
// }
