
pub mod cal;
pub mod parser_cmd;
pub mod task;

pub const DEFAULT_FILE: &str = "./caldata.json";
// #[macro_use] extern crate lalrpop_util;
pub mod task_item;
// lalrpop_mod!(pub task_item);

pub enum Error {
    TDReadError,
    TDParseError(String),
    TDQuit
}