use super::item::NewItem;
use chrono::{TimeZone, DateTime, NaiveDateTime, Utc};

use serde::Deserialize;

// For retrieving from API calls
#[derive(Deserialize, Debug)]
pub struct NewItemTz {
  pub id: Option<i32>,
  pub start_d: Option<DateTime<Utc>>,
  pub end_d: Option<DateTime<Utc>>,
  pub repeats: Option<String>,
  pub title: String,
  pub note: Option<String>,
  pub marked_done: Option<bool>, // I'm not sure if we need this
  pub deleted: Option<bool>,
  pub journal: Option<bool>,
  pub todo: Option<bool>,
  pub cal: Option<bool>,
  pub user_id: Option<i32>,

  pub tags: Vec<String>,
  pub refs: Vec<super::reference::NewItemRef>
}

// I don't understand how this type signature works, it feels like I'm casting the type backwards from the argument? is that allowed? Compiler says it's ok so ¯\_(ツ)_/¯
// I think this works because DateTime _requires_ a tz, but it doesn't matter what it is, the functions all work agnostic of the tz
pub fn opt_utc_to_naive<Tz: TimeZone>(dt_tz: Option<DateTime<Tz>>) -> Option<NaiveDateTime> {
  dt_tz.map(|date| date.naive_utc())
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
      marked_done: old_item.marked_done,
      deleted: old_item.deleted,
      journal: old_item.journal,
      todo: old_item.todo,
      cal: old_item.cal,
      user_id: old_item.user_id
    }
  }
}
