use std::fmt::{Display, self, Debug};
use std::error::Error;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use crate::util::errors::TodoError;

/**
 * DieselError is used to wrap and pass a diesel error.
 */
pub struct DieselError {
  source_error: diesel::result::Error
}

impl DieselError {
  pub fn from_err(e: diesel::result::Error) -> DieselError {
    DieselError { source_error: e }
  }
}

impl TodoError for DieselError {}

impl Display for DieselError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.source() {
      Some(e) => write!(f, "An error occurred with Diesel: {}", e),
      None => write!(f, "An error occurred with Diesel")
    }
  }
}

impl Debug for DieselError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

impl ResponseError for DieselError {
  fn status_code(&self) -> StatusCode {
    StatusCode::BAD_REQUEST
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponse::BadRequest().body(self.to_string()).into()
  }
}

impl Error for DieselError {
  /**
   * returns Option<Error>, since every instance of DieselError will be written by me it should be ok to always unwrap this, but better safe than sorry.
   */
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(&self.source_error)
  }
}

impl From<diesel::result::Error> for DieselError {
  fn from(e: diesel::result::Error) -> Self {
    Self { source_error: e }
  }
}

/**
 * so that this error can be used for the general Box<dyn TodoError>
 * TODO: at some point I want to make TodoResult<T> <= Result<T, Box<dyn TodoError>> or something the standard across my codebase, that would be nice
 */
impl From<DieselError> for Box<dyn TodoError> {
  fn from(e: DieselError) -> Box<dyn TodoError + 'static> {
    Box::new(e)
  }
}
