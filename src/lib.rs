use std::{env};
pub mod cal;
pub mod async_direct_cmd;
pub mod parser_help;

pub const DEFAULT_FILE: &str = "./caldata.json";
pub mod task_item;
pub mod old_task;
use serde_json;
use chrono::NaiveDate;
use cal::Repetition;
use url;

// diesel schemas
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
// use std::env;

pub mod schema;
// pub mod models;
pub mod models;

#[derive(Debug)]
pub enum TDError {
    IOError(String),
    ParseError(String),
    PostgresError(String),
    HyperError(String),
    VarError(String),
    ConnectionError(String),
    NoneError,
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
      Self::IOError(v) // replace this with a wildcard
        | Self::ParseError(v)
        | Self::PostgresError(v)
        | Self::VarError(v)
        | Self::ConnectionError(v)
        | Self::HyperError(v) => v,
      Self::NoneError => "'None' tried to unwrap!",
      Self::SerializeError => "Serde couldn't serialize",
      Self::Quit => "Quit Action"
    };
    write!(f, "{}", value)
  }
}

// It would be sick as hell to get a macro to do this for me
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

// impl From<std::option::NoneError> for TDError {
//   fn from (_error: std::option::NoneError) -> Self {
//     TDError::NoneError
//   }
// }

impl From<url::ParseError> for TDError {
  fn from (error: url::ParseError) -> Self {
    let value = format!("Can't parse url! {}", error);
    TDError::ParseError(value)
  }
}

impl From<std::num::ParseIntError> for TDError {
  fn from (error: std::num::ParseIntError) -> Self {
    let value = format!("{}", error);
    TDError::ParseError(value)
  }
}

impl From<std::env::VarError> for TDError {
  fn from (error: std::env::VarError) -> Self {
    let value = format!("{}", error);
    TDError::VarError(value)
  }
}

impl From<diesel::result::Error> for TDError {
  fn from (error: diesel::result::Error) -> Self {
    let value = format!("{}", error);
    TDError::ConnectionError(value)
  }
}

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
      .expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
      .expect(&format!("Error connecting to {}", database_url))
}

pub trait TaskLike {
  fn get_date(&self) -> NaiveDate;
  fn get_rep(&self) -> Repetition;
  fn is_finished(&self) -> bool;
  fn get_last_completed(&self) -> Option<&NaiveDate>;
  fn to_string(&self) -> String;
}
