extern crate chrono;
// use chrono::prelude::*; // Utc, Local
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use crate::cal::calendar::{Repetition};
use crate::cal::calendar;
use std::fmt;
// use crate::cal::calendar;
static mut NEXT_ID: u32 = 1;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskItem {
    id: u32,
    pub start: NaiveDate,
    pub repetition: Repetition,
    pub title: String,
    pub note: String,
    pub completed: Vec<NaiveDate>,
    pub finished: bool,
}

#[derive(Debug)]
pub enum Mods {
    Start(Vec<u32>),
    Rep(calendar::Repetition),
    Title(String),
    Note(String),
}

impl TaskItem {
    pub unsafe fn new(start: NaiveDate, title: String, note: String, rep: Repetition) -> TaskItem {
        NEXT_ID += 1;
        TaskItem {
            id: NEXT_ID,
            start,
            repetition: rep,
            title: title,
            note: note,
            finished: false,
            completed: vec![],
        }
    }
    pub unsafe fn set_id_start(highest: u32) {
        NEXT_ID = highest;
    }

    pub unsafe fn from_raw(raw: RawTaskItem) -> Option<TaskItem> {
        let start = calendar::get_start(raw.start)?;
        let task = TaskItem::new(start, raw.title, raw.note, raw.repetition);
        Some(task)
    }

    pub fn apply_modifications(&mut self, mods: Vec<Mods>) {
        for m in mods {
            match m {
                Mods::Start(raw_start) => match calendar::get_start(raw_start.to_vec()) {
                    Some(start) => self.start = start,
                    None => println!("Can't make {:?} into a date!", raw_start),
                },
                Mods::Rep(rep) => self.repetition = rep,
                Mods::Title(title) => self.title = title,
                Mods::Note(note) => self.note = note,
            }
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl fmt::Display for TaskItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}: {}, {rep}\nNotes: {note}",
            id=self.get_id(),
            title=self.title,
            start=self.start,
            rep=self.repetition,
            note=self.note)
    }
}

#[derive(Debug)]
pub struct RawTaskItem {
    pub start: Vec<u32>,
    pub repetition: Repetition,
    pub title: String,
    pub note: String,
    pub finished: bool,
}

impl RawTaskItem {
    pub fn new_empty() -> RawTaskItem {
        RawTaskItem {
            start: vec![],
            repetition: Repetition::Weekly,
            title: String::from("Title"),
            note: String::from(""),
            finished: false,
        }
    }
}
