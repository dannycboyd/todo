use chrono::NaiveDate;
use crate::Repetition;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt;

use crate::schema::{tasks, task_completions};

#[derive(Queryable, Identifiable)]
#[table_name = "tasks"]
// super important note: THESE FIELDS MUST BE IN THE SAME ORDER AS THE `schema.rs` LISTINGS
// If you don't, then you'll get serialization errors about cross-casting types. BAD
pub struct Task {
  id: i32,
  pub start: NaiveDate,
  pub repetition: String,
  pub title: String,
  pub note: String,
  pub finished: bool,
}

impl super::TaskLike for Task {
  fn get_date(&self) -> NaiveDate {
      self.start
  }

  fn get_rep(&self) -> Repetition {
    Repetition::from_str(&self.repetition).unwrap_or_else(|_| Repetition::Never)
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
    write!(f, "{} - {}: {}, {rep}\nNotes: {note}\nFinished: {finished}",
        id=self.id,
        title=self.title,
        start=self.start,
        rep=self.repetition,
        note=self.note,
        finished=self.finished)
  }
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Task)]
#[table_name = "task_completions"]
pub struct Completion {
  id: i32,
  date: NaiveDate,
  task_id: i32
}

impl Completion {
  pub fn get_date(&self) -> &NaiveDate {
    &self.date
  }
}