use std::fmt;

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
  fn response(&self) -> Option<AppResponse>;
  fn status_code(&self) -> i32;
}

impl TodoError {}

impl fmt::Display for TodoError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "")
  }
}
