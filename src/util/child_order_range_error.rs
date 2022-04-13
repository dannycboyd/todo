use std::fmt::{Display, self, Debug};
use std::error::Error;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use crate::util::errors::TodoError;

/**
 * ChildOrderRangeError is used when the client attempts to set an item's `child_order` to an invalid value, such as attempting to call shift_up on an item with a child_order of 0 or attempting to set an item to the same child_order as a sibling
 */
pub struct ChildOrderRangeError {}

impl TodoError for ChildOrderRangeError {}

impl Display for ChildOrderRangeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Attempted to change `child_order` to bad value (duplicate, less than zero or greater than children.length)")
  }
}

impl Debug for ChildOrderRangeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Attempted to change `child_order` to bad value (duplicate, less than zero or greater than children.length)")
  }
}

impl ResponseError for ChildOrderRangeError {
  fn status_code(&self) -> StatusCode {
    StatusCode::BAD_REQUEST
  }
  fn error_response(&self) -> HttpResponse {
    HttpResponse::BadRequest().body(self.to_string()).into()
  }
}

impl Error for ChildOrderRangeError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

impl From<ChildOrderRangeError> for Box<dyn TodoError> {
  fn from(e: ChildOrderRangeError) -> Box<dyn TodoError + 'static> {
    Box::new(e)
  }
}
