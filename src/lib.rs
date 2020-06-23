
// #![feature(try_trait, async_closure)]
use std::{env, fmt};
pub mod cal;
pub mod parser_cmd;
pub mod async_direct_cmd;
pub mod task;
pub mod parser_help;

pub const DEFAULT_FILE: &str = "./caldata.json";
// #[macro_use] extern crate lalrpop_util;
pub mod task_item;
use task::TaskItem;
// use lalrpop_util::ParseError;
use serde_json;
use chrono::NaiveDate;
use cal::Repetition;
use url;

use tokio_postgres::Row;

// diesel schemas
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
// use std::env;

pub mod schema;
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

fn from_row(row: Row) -> Result<TaskItem, TDError> {
  let id: i32 = row.try_get(0)?;
  let date: NaiveDate = row.try_get("start")?;
  let rep: &str = row.try_get("repeats")?;
  let rep = rep.parse::<Repetition>()?;
  let title: &str = row.try_get("title")?;
  let note: &str = match row.try_get("note") {
      Ok(n) => n,
      Err(_e) => ""
  };
  let finished: bool = row.try_get("finished")?;
  Ok(TaskItem::new_by_id(id, date, title.to_string(), note.to_string(), rep, finished))
}

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
      .expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
      .expect(&format!("Error connecting to {}", database_url))
}

pub fn connection_info() -> Result<String, TDError> {

  let mut has_dbname = false;
  let mut has_user = false;
  let mut has_host = false;

  let args: Vec<String> = env::args().collect();
  let mut db_string: String = String::new();
  for (i, arg) in args.iter().enumerate() {
      if i > 0 {
          if !has_dbname && arg.contains("dbname") {
              db_string.push_str(arg);
              db_string.push(' ');
              has_dbname = true;
          }
          if !has_user && arg.contains("user") {
              db_string.push_str(arg);
              db_string.push(' ');
              has_user = true;
          }
          if !has_host && arg.contains("host") {
              db_string.push_str(arg);
              db_string.push(' ');
              has_host = true;
          }
      }
  }

  if !has_dbname {
    println!("Getting TODO_DBNAME from environment...");
    let dbname = env::var("TODO_DBNAME")?;
    let dbname = format!("dbname={} ", dbname);
    db_string.push_str(&dbname);
  }
  if !has_user {
    println!("Getting TODO_USERNAME from environment...");
    let dbuser = env::var("TODO_USERNAME")?;
    let dbuser = format!("user={} ", dbuser);
    db_string.push_str(&dbuser);

  }
  if !has_host {
    println!("Getting TODO_HOSTNAME from environment...");
    let dbhost = env::var("TODO_HOSTNAME")?;
    let dbhost = format!("host={}", dbhost);
    db_string.push_str(&dbhost);
  }
  Ok(db_string)
}

pub trait TaskLike {
  fn get_date(&self) -> NaiveDate;

  fn get_rep(&self) -> Repetition;

  fn is_finished(&self) -> bool;
  fn get_last_completed(&self) -> Option<&NaiveDate>;

  fn to_string(&self) -> String;
}
