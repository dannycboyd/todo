use chrono::NaiveDate;

#[derive(Queryable)]
pub struct Notes {
  id: i32,
  created_at: NaiveDate,
  updated_at: Option<NaiveDate>,
  body: String
}
