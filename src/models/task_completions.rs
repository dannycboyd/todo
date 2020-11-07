use chrono::NaiveDate;

// use super::models::task::Task;
use crate::models::task::Task;
use crate::schema::task_completions;

#[derive(Queryable, Identifiable, Associations, Insertable)]
#[belongs_to(Task)]
#[table_name = "task_completions"]
pub struct Completion {
  id: i32,
  task_id: i32,
  date: NaiveDate
}

impl Completion {
  pub fn get_date(&self) -> &NaiveDate {
    &self.date
  }
}

#[derive(Insertable)]
#[table_name = "task_completions"]
pub struct NewCompletion {
  pub task_id: i32,
  pub date: NaiveDate
}

impl NewCompletion {
  pub fn new(id: i32, date: NaiveDate) -> Self {
    Self {
      task_id: id,
      date: date
    }
  }
}
