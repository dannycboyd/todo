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
        let mut me = Cmd { cmd_raw: String::new(), storage: vec![], parser: task_item::CmdParser::new() };
        me.load(DEFAULT_FILE);
        me
    }

    pub fn handle_load(&mut self, url: &str) {
        match self.load(url) {
            Ok(count) => println!("Loaded {} tasks {}", count, url),
            Err(e) => println!("Error occurred during file load: {:?}", e),
        }
    }

    fn load(&mut self, url: &str) -> Result<usize, Box<error::Error>> {
        let file = fs::read_to_string(url)?;
        self.storage = serde_json::from_str(&file)?;
        let len = self.storage.len();
        if len > 0 {
            unsafe {
                TaskItem::set_id_start(self.storage[len - 1].get_id());
            }
        }
        Ok(len)
    }

    pub fn handle_save(&self, url: &str) {
        match self.save(url) {
            Ok(count) => println!("Saved {} tasks", count),
            Err(e) => println!("Error occurred during save: {:?}", e),
        }
    }

    pub fn save(&self, url: &str) -> Result<usize, Box<error::Error>> {
        if self.storage.len() > 0 {
            let mut file = File::create(DEFAULT_FILE)?;
            let contents = serde_json::to_string(&self.storage)?;
            write!(file, "{}", contents);
        };
        Ok(self.storage.len())
    }

    fn unwrap_date(raw: Option<Vec<u32>>) -> Option<NaiveDate> {
        let date_raw = raw?;
        calendar::get_start(date_raw)
    }

    fn show(&self, kind: calendar::Repetition, date_raw: Option<Vec<u32>>) {
        let start = calendar::date_or_today(date_raw);
        calendar::show_type(kind, start, &self.storage);
    }

    fn list_all(&self) {
        for t in self.storage.iter() {
            println!("{}", t.to_string());
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
            Args::List => self.list_all(),
            Args::Save => self.handle_save(DEFAULT_FILE),
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
