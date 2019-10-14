
pub mod cal;
pub mod parser_cmd;
pub mod task;

pub const DEFAULT_FILE: &str = "./caldata.json";
// #[macro_use] extern crate lalrpop_util;
pub mod task_item;
use lalrpop_util::ParseError;

pub enum TDError {
    IOError(String),
    ParseError(String),
    SerializeError,
    Quit,
}

// impl From<ParseError<usize, (usize, &str), ()>> for Error {
//   fn from(error: ParseError<usize, (usize, &str), ()>) -> Self {
//     let value = format!("{}", error);
//     Error::TDParseError(value)
//   }
// }

impl From<std::io::Error> for TDError {
  fn from (error: std::io::Error) -> Self {
    let value = format!("{}", error);
    TDError::IOError(value)
  }
}