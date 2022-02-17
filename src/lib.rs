use std::{env};
pub mod cal;
pub mod async_direct_cmd;
pub mod parser_help;

pub const DEFAULT_FILE: &str = "./caldata.json";
pub mod task_item;
pub mod old_task;
use actix_web::web;
use r2d2::PooledConnection;
use serde_json;
use chrono::NaiveDate;
use cal::Repetition;
use diesel::{prelude::*, r2d2::ConnectionManager};
use diesel::pg::PgConnection;
use url;

extern crate chrono;
extern crate argon2;

// diesel schemas
#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod actions; // actix-web action functions
pub mod routes;
pub mod util;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;
pub fn get_pool_connection(pool: web::Data<DbPool>) -> DbConn {
  pool.get().expect("Couldn't get connection from pool!")
}
#[derive(Debug)]

// * REDO ERRORS AS SEPARATE STRUCTS WITH A SINGLE APPERROR TYPE -> TodoError trait in util/errors.rs
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

use actix_web::{dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse};
impl error::ResponseError for TDError {
  fn error_response(&self) -> HttpResponse {
    println!("This is an error! {}", *self);
    HttpResponseBuilder::new(self.status_code())
      .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
      .body(self.to_string())
  }

  fn status_code(&self) -> StatusCode {
    match self {
      _ => StatusCode::BAD_REQUEST,
    }
  }
}

// It would be sick as hell to get a macro to do this for me
impl From<serde_json::error::Error> for TDError {
  fn from(error: serde_json::error::Error) -> Self {
    let value = format!("Serde Parsing Error: {}", error);
    TDError::ParseError(value)
  }
}

impl From<diesel_migrations::RunMigrationsError> for TDError {
  fn from(error: diesel_migrations::RunMigrationsError) -> Self {
    TDError::PostgresError(format!("Error running migrations!\n{}", error))
  }
}

impl From<std::io::Error> for TDError {
  fn from(error: std::io::Error) -> Self {
    let value = format!("{}", error);
    TDError::IOError(value)
  }
}

impl From<std::string::FromUtf8Error> for TDError {
  fn from(error: std::string::FromUtf8Error) -> Self {
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
  fn from(error: url::ParseError) -> Self {
    let value = format!("Can't parse url! {}", error);
    TDError::ParseError(value)
  }
}

impl From<std::num::ParseIntError> for TDError {
  fn from(error: std::num::ParseIntError) -> Self {
    let value = format!("{}", error);
    TDError::ParseError(value)
  }
}

impl From<std::env::VarError> for TDError {
  fn from(error: std::env::VarError) -> Self {
    let value = format!("{}", error);
    TDError::VarError(value)
  }
}

impl From<diesel::result::Error> for TDError {
  fn from(error: diesel::result::Error) -> Self {
    let value = format!("{}", error);
    TDError::ConnectionError(value)
  }
}

pub fn establish_connection() -> PgConnection {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

/**
  trait TaskLike
  Currently, used both for display (for printing in cal) and for filtering in cal.
  Unfortunately, this usage is split across two structs, and this trait should become two traits.
  an alternative solution would be to remove the traits and have their references use specific types instead.
  this would limit duplicate definition/implementation and allow for explicit typing, which I think I want.
*/
pub trait TaskLike {
  fn get_id(&self) -> i32;
  fn get_start(&self) -> Option<NaiveDate>;
  fn formatted_date(&self) -> String;
  fn get_rep(&self) -> Repetition;
  fn is_finished(&self) -> bool;
  fn get_last_completed(&self) -> Option<&NaiveDate>;
  fn to_string(&self) -> String;
}

pub trait ItemTrait1 {
  fn get_id(&self) -> i32;
  fn get_start(&self) -> Option<NaiveDate>;
  fn get_rep(&self) -> Repetition;
  fn is_finished(&self) -> bool;
  fn get_last_completed(&self) -> Option<&NaiveDate>;
}

pub trait ItemTrait2 {
  fn formatted_date(&self) -> String;
  fn to_string(&self) -> String;
}

// TESTING_TODO: this should get a test
// note: good grief trying to write a test for this got me pulling my hair out.
// maybe replace with this https://github.com/actix/examples/blob/master/json/json_decode_error/src/main.rs
type TodoResult<T> = Result<T, actix_web::error::Error>;
const MAX_SIZE: usize = 262_144; // get this from env somehow?
use serde::de::DeserializeOwned;
// from https://github.com/actix/examples/blob/master/json/json/src/main.rs
// takes a bytestream Payload and a type that implements Deserialize and returns a result of that type.
pub async fn parse_json<T: DeserializeOwned>(mut payload: web::Payload) -> TodoResult<T> {
  use futures::StreamExt;
  let mut body = web::BytesMut::new();

  while let Some(chunk) = payload.next().await {
    let chunk = chunk?;
    // limit max size of in-memory payload
    if (body.len() + chunk.len()) > MAX_SIZE {
      return Err(actix_web::error::ErrorBadRequest("overflow")); // tweak this?
    }
    body.extend_from_slice(&chunk);
  }

  serde_json::from_slice::<T>(&body).map_err(|e| {
    // do some wacky stuff here
    eprintln!("An error occurred in todo_service::parse_json:\n{}", e);
    let res = format!("Invalid upload request: {}", e);
    actix_web::error::ErrorBadRequest(res)
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::models::item::RefsItem;

  use actix_web::{http, test, App};

  fn get_new_refs_item(title: &str) -> RefsItem {
    RefsItem {
      id: None,
      created_at: None,
      updated_at: None,
      start_d: None,
      end_d: None,
      repeats: None,
      title: Some(String::from(title)),
      note: None,
      marked_done: None,
      deleted: None,
      parent_id: None,
      journal: None,
      todo: None,
      cal: None,
      user_id: None,
      references: vec![],
      tags: vec![],
      child_order: None,
    }
  }

  async fn test_parse_refs_item(body: web::Payload) -> Result<HttpResponse, http::Error> {
    let foo = parse_json::<RefsItem>(body).await;
    Ok(match foo {
      Ok(item) => {
        let expected_item = get_new_refs_item("test_item");
        assert_eq!(expected_item.title, item.title); // ehhhhh this should be a more real test but w.e
        HttpResponse::Ok().finish()
      }
      Err(e) => {
        println!("help");
        HttpResponse::InternalServerError().finish()
      }
    })
  }

  #[actix_rt::test]
  async fn test_parse_item() {
    // make a web::Payload with some json inside it
    // this is the clunkiest piece ever, all I want to do is construct a web::Payload BUT THERE'S NO WAY TO DO THAT so I gotta stand up the whole
    // actix server harness and run it like a little server in order to even check if this function works! At this rate why bother, just test one of the endpoints with a mocked actions handler
    let item_json_string =
      String::from("{\"title\":\"test_item\", \"references\": [], \"tags\": []}");
    let mut app =
      test::init_service(App::new().route("/", web::get().to(test_parse_refs_item))).await;

    let req = test::TestRequest::get()
      .uri("/")
      .set_payload(item_json_string)
      .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
    // assert_eq!(resp.status_code)
  }

  #[actix_rt::test]
  async fn test_parse_item_fail() {
    // this string can't be parsed as a RefsItem, so the test expects to see an error
    // this error should be a client error, because the data is bad, not the server
    let item_json_string =
      String::from("{\"fail\": \"there's no references or tags on this item, so it will fail\" }");
    let mut app =
      test::init_service(App::new().route("/", web::get().to(test_parse_refs_item))).await;

    let req = test::TestRequest::get()
      .uri("/")
      .set_payload(item_json_string)
      .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_server_error());
  }
}
