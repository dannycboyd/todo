// command line crate
use std::io;
use std::io::{Write};
use std::cmp::min;
use serde_json;
use std::fs::File;
use std::error;

use std::fs;

use crate::task::TaskItem;
use crate::cal::calendar;
use crate::DEFAULT_FILE;

use lalrpop_util;
lalrpop_mod!(pub task_item);

#[derive(Debug)]
pub enum Args {
    Make(Vec<u32>, String, String, calendar::Repetition),
    Test(Vec<u32>),
    Test2(Vec<u32>, calendar::Repetition)
}

pub struct Cmd {
    // pub cmd: Vec<String>,
    cmd_raw: String,
    storage: Vec<TaskItem>,
    parser: task_item::CmdParser,
    // pub last: Option<&TaskItem>,
}

impl Cmd {
    pub fn new() -> Cmd {
        Cmd { cmd_raw: String::new(), storage: vec![], parser: task_item::CmdParser::new() }
    }

    fn parse_cmd(&self) {
        match self.parser.parse(&self.cmd_raw) {
            Ok(cmd) => {
                println!("parsed command: {:?}", cmd);
            },
            Err(e) => {
                println!("An error occurred: {}", e);
            }
        }
    }

    pub fn exec(&mut self) {
        loop {
            self.cmd_raw = String::new();
            match io::stdin().read_line(&mut self.cmd_raw) {
                Err(e) => println!("An error occurred reading line: {:?}", e),
                Ok(_len) => {
                    self.parse_cmd();
                }
            }
        }
    }

}
