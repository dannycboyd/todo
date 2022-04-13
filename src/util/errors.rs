use std::fmt;
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::{StatusCode};
use std::error::Error;

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

pub trait TodoError: fmt::Display + fmt::Debug + ResponseError + Error + Send {}

#[derive(Clone)]
pub struct DefaultError {
  code: u16,
  body: String
}

impl TodoError for DefaultError {}

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

impl ResponseError for DefaultError {
  fn status_code(&self) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponse::InternalServerError().into()
  }
}

impl Error for DefaultError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

impl From<DefaultError> for Box<dyn TodoError> {
  fn from(e: DefaultError) -> Box<dyn TodoError + 'static> {
    Box::new(e)
  }
}
