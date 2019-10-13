// command line crate
use std::io;
use std::io::{Write};
// use std::cmp::min;
use serde_json;
use std::fs::File;
use std::fs;
use std::error;
// use chrono::NaiveDate;

use super::task::{ TaskItem, RawTaskItem, Mods };
use super::cal;
use super::DEFAULT_FILE;
// use crate::print;

/**
 * this file needs to be refactored into library functions and command parsing functions
 */

// lalrpop_mod!(pub task_item);
use super::task_item;
#[derive(Debug)]
pub enum Args {
    Do(u32),
    Finish(u32),
    Help,
    List,
    MakeRaw(RawTaskItem),
    Mods(u32, Vec<Mods>),
    Save,
    Show(cal::Repetition, Option<Vec<u32>>)
}

pub struct Cmd {
    cmd_raw: String,
    storage: Vec<TaskItem>,
    parser: task_item::CmdParser,
}

impl Cmd {
    pub fn new() -> Cmd {
        let mut me = Cmd { cmd_raw: String::new(), storage: vec![], parser: task_item::CmdParser::new() };
        me.handle_load(DEFAULT_FILE);
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
            let mut file = File::create(url)?;
            let contents = serde_json::to_string(&self.storage)?;
            match write!(file, "{}", contents) {
                Ok(foo) => println!("Saved!"),
                Err(e) => println!("An error occurred! {:?}", e)
            };
        };
        Ok(self.storage.len())
    }

    fn show(&self, kind: cal::Repetition, date_raw: Option<Vec<u32>>) {
        let start = cal::date_or_today(date_raw);
        cal::show_type(kind, start, &self.storage);
    }

    fn list_all(&self) {
        for t in self.storage.iter() {
            println!("{}\n", t.to_string());
        }
    }

    fn find_task_by_id(&mut self, id: u32) -> Option<&mut TaskItem> {
        self.storage.iter_mut().find(|task| task.get_id() == id)
    }

    fn modify(&mut self, id: u32, cmds: Vec<Mods>) {
        match self.find_task_by_id(id) {
            Some(task) => {
                task.apply_modifications(cmds);
            },
            None => println!("No task exists with id {}!", id),
        }
    }

    fn do_task(&mut self, id: u32) {
        match self.find_task_by_id(id) {
            Some(task) => {
                println!("Mark done today");
                task.mark_completed(cal::date_or_today(None));
                println!("Done! {:?}", task)
            },
            None => println!("Can't find task with id {}", id),
        }
    }

    fn finish_task(&mut self, id: u32) {
        match self.find_task_by_id(id) {
            Some(task) => {
                println!("Mark finished today");
                task.mark_finished(cal::date_or_today(None));
                println!("Finished {}", task)
            },
            None => println!("Can't find task with id {}", id),
        }
    }

    // this part needs to be moved

    fn do_cmd(&mut self, cmd: Args) {
        match cmd {
            Args::MakeRaw(raw) => unsafe { // move this into a new function
                match TaskItem::from_raw(raw) {
                    None => println!("An error occurred, likely the dates couldn't be parsed into real dates"),
                    Some(task) => {
                        println!("New task: {:?}", task);
                        self.storage.push(task);
                    },
                }
            },
            Args::Do(id) => self.do_task(id),
            Args::Finish(id) => self.finish_task(id),
            Args::Mods(id, cmds) => self.modify(id, cmds),
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
