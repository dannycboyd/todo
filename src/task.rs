extern crate chrono;
// use chrono::prelude::*; // Utc, Local
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use crate::cal::{Repetition};
use crate::cal;
use std::fmt;
use std::str::FromStr;
// use crate::cal::calendar;
static mut NEXT_ID: i32 = 1;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskItem {
    id: i32,
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
    Rep(cal::Repetition),
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
            title,
            note,
            finished: false,
            completed: vec![],
        }
    }
    pub unsafe fn set_id_start(highest: i32) {
        NEXT_ID = highest;
    }

    pub fn new_by_id(id: i32, start: NaiveDate, title: String, note: String, rep: Repetition, finished: bool) -> TaskItem {
        TaskItem {
            id,
            start,
            repetition: rep,
            title,
            note,
            finished,
            completed: vec![],
        }
    }

    pub unsafe fn from_raw(raw: RawTaskItem) -> Option<TaskItem> {
        let start = cal::get_start(raw.start)?;
        let task = TaskItem::new(start, raw.title, raw.note, raw.repetition);
        Some(task)
    }

    pub fn apply_modifications(&mut self, mods: Vec<Mods>) {
        for m in mods {
            match m {
                Mods::Start(raw_start) => match cal::get_start(raw_start.to_vec()) {
                    Some(start) => self.start = start,
                    None => println!("Can't make {:?} into a date!", raw_start),
                },
                Mods::Rep(rep) => self.repetition = rep,
                Mods::Title(title) => self.title = title,
                Mods::Note(note) => self.note = note,
            }
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn set_id(&mut self, new_id: i32) {
        self.id = new_id;
    }

    pub fn mark_completed(&mut self, day: NaiveDate) {
        let mut i: usize = 0;
        while i < self.completed.len() {
            let curr = self.completed[i];
            if curr == day {
                println!("Already done on {}", day);
                return;
            } else if curr < day {
                self.completed.insert(i, day);
                println!("Inserted at index {}", i);
                return;
            }
            i += 1;
        }
        println!("put it at the end");
        self.completed.push(day)
    }

    pub fn mark_finished(&mut self, day: NaiveDate) {
        self.mark_completed(day);
        self.finished = true;
    }

    pub fn done_on_day(&self, day: NaiveDate) -> bool {
        match self.completed.iter().find(|d| *d == &day) {
            Some(_d) => true,
            None => false,
        }
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
