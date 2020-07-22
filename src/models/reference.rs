use chrono::NaiveDate;

#[derive(Queryable)]
pub struct Ref {
  created_at: NaiveDate,
  parent_task: i32,
  parent_note: i32,
  child_task: i32,
  child_note: i32
}
