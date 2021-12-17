use std::fmt;
use actix_web::HttpResponse;
use actix_web::http::{StatusCode};
// use actix_http::{Response};

// I want this to replace TDError
//

/*
  Useable in the server event loop
  set appropriate return status
  set up a catch or something at the top level which logs the error, and returns a user-appropriate response using this trait
*/

/*
pub enum TDError {
  IOError(String), - unused by the service
  ParseError(String), - unused by the service
  PostgresError(String), - 500
  HyperError(String), - 400
  VarError(String), - 400
  ConnectionError(String), - 500
  NoneError, - unused by the service, anything like this ought to be made more verbose
  SerializeError, - unused by the service
  Quit
}
*/

pub trait TodoError: fmt::Display + fmt::Debug {
  fn generate_http_response(&self) -> HttpResponse;
  fn status_code(&self) -> u16;
}

// impl TodoError {}

// impl fmt::Display for TodoError {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     write!(f, "")
//   }
// }

pub struct DefaultError {
  code: u16,
  body: String
}

impl TodoError for DefaultError {
  fn generate_http_response(&self) -> HttpResponse {
    HttpResponse::build(StatusCode::from_u16(self.code).unwrap()).body(format!("{}", self.body))
  }

  fn status_code(&self) -> u16 {
    self.code
  }
}

impl fmt::Display for DefaultError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "TodoError - Code {}\n{}\n", self.code, self.body)
  }
}

impl fmt::Debug for DefaultError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "TodoError {{ code: {}, body: {} }}",
      self.code, self.body
    )
  }
}

/* ok this is something
how do I want to handle this?
many error structs which all handle their own thing
a single type?

if there's an error with json response, maybe want to handle that
  is this the case? idk
  turn the regular into a json response?
  what kind of errors am I getting?
  the big ones are user error
    bad request
    bad Item data (duplicate title, wrong ID, dates in the wrong order?)
      be specific about this

*/
