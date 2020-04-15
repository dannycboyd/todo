// command line crate. Works on local storage .json. I want a similar API sort of thing like this but for the server.
use std::io::{Write};

use serde_json;
use std::fs::File;
use std::fs;
use std::error;

use super::task::{ TaskItem, RawTaskItem, Mods };
use super::{cal, TDError};
use super::DEFAULT_FILE;

#[derive(Debug)]
pub enum Args {
    Do(i32),
    Finish(i32),
    Help,
    List,
    MakeRaw(RawTaskItem),
    Mods(i32, Vec<Mods>),
    Save,
    Show(cal::Repetition, Option<Vec<u32>>),
    Quit,
}

pub struct Cmd {
    storage: Vec<TaskItem>,
}

impl Cmd {
    pub fn new() -> Cmd {
        let mut me = Cmd { storage: vec![] };
        me.handle_load(DEFAULT_FILE);
        me
    }

    pub fn handle_load(&mut self, url: &str) {
        match self.load(url) {
            Ok(count) => println!("Loaded {} tasks {}", count, url),
            Err(e) => println!("Error occurred during file load: {:?}", e),
        }
    }

    fn load(&mut self, url: &str) -> Result<usize, Box<dyn error::Error>> {
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

    pub fn save(&self, url: &str) -> Result<(), TDError> {
        if self.storage.len() > 0 {
            let mut file = File::create(url)?;
            let contents = serde_json::to_string(&self.storage)?;
            write!(file, "{}", contents)?;
            Ok(())
        } else {
            Err(TDError::IOError(String::from("No items to save"))) // 1 - empty
        }
    }

    pub fn show(&self, kind: cal::Repetition, date_raw: Option<Vec<u32>>) {
        let start = cal::date_or_today(date_raw);
        cal::show_type(kind, start, &self.storage);
    }

    pub fn list_all(&self) {
        for t in self.storage.iter() {
            println!("{}\n", t.to_string());
        }
    }

    fn find_task_by_id(&mut self, id: i32) -> Option<&mut TaskItem> {
        self.storage.iter_mut().find(|task| task.get_id() == id)
    }

    pub fn modify(&mut self, id: i32, cmds: Vec<Mods>) {
        match self.find_task_by_id(id) {
            Some(task) => {
                task.apply_modifications(cmds);
            },
            None => println!("No task exists with id {}!", id),
        }
    }

    pub fn do_task(&mut self, id: i32) {
        match self.find_task_by_id(id) {
            Some(task) => {
                println!("Mark done today");
                task.mark_completed(cal::date_or_today(None));
                println!("Done! {:?}", task)
            },
            None => println!("Can't find task with id {}", id),
        }
    }

    pub fn finish_task(&mut self, id: i32) {
        match self.find_task_by_id(id) {
            Some(task) => {
                println!("Mark finished today");
                task.mark_finished(cal::date_or_today(None));
                println!("Finished {}", task)
            },
            None => println!("Can't find task with id {}", id),
        }
    }

    pub fn make_raw(&mut self, _raw: RawTaskItem) {
        // unsafe {
        //     match TaskItem::from_raw(raw) {
        //         None => println!("An error occurred parsing the raw task item. Likely an issue with the dates"),
        //         Some(task) => {
        //             println!("New task: {:?}", task);
        //             self.storage.push(task);
        //         }
        //     }
        // }
    }
}
