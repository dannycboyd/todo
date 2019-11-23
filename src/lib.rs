
pub mod cal;
pub mod parser_cmd;
pub mod task;

pub const DEFAULT_FILE: &str = "./caldata.json";
// #[macro_use] extern crate lalrpop_util;
pub mod task_item;
// use lalrpop_util::ParseError;
use serde_json;

#[derive(Debug)]
pub enum TDError {
    IOError(String),
    ParseError(String),
    PostgresError(String),
    HyperError(String),
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
    let value = match self {
      Self::IOError(v) | Self::ParseError(v) | Self::PostgresError(v) | Self::HyperError(v) => v,
      Self::SerializeError => "Serde couldn't serialize",
      Self::Quit => "Quit Action",
      _ => "Generic Error Message"
    };
    write!(f, "{}", value)
  }
}

impl From<serde_json::error::Error> for TDError {
  fn from (error: serde_json::error::Error) -> Self {
    let value = format!("Serde Parsing Error: {}", error);
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

impl From<hyper::Error> for TDError {
  fn from (error: hyper::Error) -> Self {
    let value = format!("{}", error);
    TDError::HyperError(value)
  }
}

impl From<std::string::FromUtf8Error> for TDError {
  fn from (error: std::string::FromUtf8Error) -> Self {
    let value = format!("FromUFT8Error: {}", error);
    TDError::ParseError(value)
  }
}