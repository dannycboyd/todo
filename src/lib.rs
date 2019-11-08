
pub mod cal;
pub mod parser_cmd;
pub mod task;

pub const DEFAULT_FILE: &str = "./caldata.json";
// #[macro_use] extern crate lalrpop_util;
pub mod task_item;
// use lalrpop_util::ParseError;
use serde_json;

pub enum TDError {
    IOError(String),
    ParseError(String),
    PostgresError(String),
    SerializeError,
    Quit,
}

impl std::error::Error for TDError {
  fn description(&self) -> &str {
    "an error"
  }
}

impl std::fmt::Display for TDError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperError is here!")
  }
}

impl std::fmt::Debug for TDError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperError is here!")
  }
}

impl From<serde_json::error::Error> for TDError {
  fn from (error: serde_json::error::Error) -> Self {
    let value = format!("{}", error);
    TDError::ParseError(value)
  }
}

impl From<std::io::Error> for TDError {
  fn from (error: std::io::Error) -> Self {
    let value = format!("{}", error);
    TDError::IOError(value)
  }
}

impl From<tokio_postgres::error::Error> for TDError {
  fn from (error: tokio_postgres::error::Error) -> Self {
    let value = format!("{}", error);
    TDError::PostgresError(value)
  }
}