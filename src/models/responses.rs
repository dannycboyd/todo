use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Response {
  pub id: Option<i32>,
  pub message: String,
  pub value: Option<i32>
}