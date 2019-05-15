// command line crate
use std::io;
use std::io::{Write};
use std::cmp::min;
use serde_json;
use std::fs::File;
use std::fs;
use std::error;
use chrono::NaiveDate;

use crate::task::TaskItem;
use crate::cal::calendar;
use crate::DEFAULT_FILE;

use lalrpop_util;
lalrpop_mod!(pub task_item);

#[derive(Debug)]
pub enum Args {
    Make(Vec<u32>, String, String, calendar::Repetition),
    Test(String),
    Show(calendar::Repetition, Option<Vec<u32>>),
    List,
    Save,
    Help
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

    fn unwrap_date(raw: Option<Vec<u32>>) -> Option<NaiveDate> {
        let date_raw = raw?;
        calendar::get_start(date_raw)
    }

    fn show(&self, kind: calendar::Repetition, date_raw: Option<Vec<u32>>) {
        match Cmd::unwrap_date(date_raw) {
            None => {
                println!("{:?}, today", kind)
            },
            Some(date) => {
                println!("{:?}, {:?}", kind, date)
            }
        }
    }

    fn do_cmd(&mut self, cmd: Args) {
        match cmd {
            Args::Make(date_raw, title, desc, rep) => {
                match calendar::get_start(date_raw) {
                    None => println!("Error: values entered can't be handled by calendar::get_start()"),
                    Some(start) => {
                        let task = unsafe {
                            TaskItem::new(start, title, desc, rep)
                        };
                        println!("new task: {:?}", task);
                        self.storage.push(task);
                    }
                }
            },
            Args::Test(val) => (),
            Args::Show(kind, when) => self.show(kind, when),
            Args::List => (),
            Args::Save => (),
            Args::Help => (),
        }
    }

    fn parse_cmd(&mut self) {
        match self.parser.parse(&self.cmd_raw) {
            Ok(cmd) => {
                println!("parsed command: {:?}", cmd);
                self.do_cmd(cmd);
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
