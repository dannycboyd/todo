use chrono::NaiveDate;
use crate::Repetition;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt;

// use crate::schema::{tasks};

// #[derive(Queryable, Identifiable)]
// #[table_name = "tasks"]
// super important note: THESE FIELDS MUST BE IN THE SAME ORDER AS THE `schema.rs` LISTINGS
// More important: schema.rs gets regenerated every time you run migrations, and shouldn't be modified by hand.
// If you don't, then you'll get serialization errors about cross-casting types. BAD
pub struct Task {
  id: i32,
  pub start: NaiveDate,
  pub repeats: String,
  pub title: String,
  pub note: String,
  pub finished: bool
}

impl crate::TaskLike for Task {
  fn get_id(&self) -> i32 {
    self.id
  }

  fn get_start(&self) -> Option<NaiveDate> {
    Some(self.start)
  }

  fn formatted_date(&self) -> String {
    self.start.to_string()
  }

  fn get_rep(&self) -> Repetition {
    Repetition::from_str(&self.repeats).unwrap_or_else(|_| Repetition::Never)
  }

  fn is_finished(&self) -> bool {
    self.finished
  }

  fn get_last_completed(&self) -> Option<&NaiveDate> {
    None
  }

  // fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
  //   self.fmt(f)
  // }

  fn to_string(&self) -> String {
    String::from(format!("{}", self))
  }
}

impl Display for Task {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{} - {}: {}, {rep}\nNotes: {note}\nFinished: {finished}",
      id = self.id,
      title = self.title,
      start = self.start,
      rep = self.repeats,
      note = self.note,
      finished = self.finished
    )
  }
}

// #[derive(Insertable, PartialEq, Deserialize)]
// #[table_name = "tasks"]
// pub struct NewTask {
//   pub start: NaiveDate,
//   pub repeats: String,
//   pub title: String,
//   pub note: String,
//   pub finished: bool
// }

// #[derive(AsChangeset)]
// #[table_name = "tasks"]
// pub struct TaskUpdate {
//   pub start: Option<NaiveDate>,
//   pub repeats: Option<String>,
//   pub title: Option<String>,
//   pub note: Option<String>,
//   pub finished: Option<bool>
// }
