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
    // completed: [DateTime<Utc>], // Should use a vector here, maybe. Other solution?
    pub finished: bool,
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
        NEXT_ID = highest + 1;
    }

    pub unsafe fn from_raw(raw: RawTaskitem) -> Option<TaskItem> {
        let start = calendar::get_start(raw.start)?;
        let mut task = TaskItem::new(start, raw.title, raw.note, raw.repetition);
        Some(task)
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title)
    }

    pub fn set_note(&mut self, note: &str) {
        // println!("set note: {}", note);
        self.note = String::from(note)
    }

    pub fn set_rep(&mut self, rep: &str) {
        match rep {
            "day" | "daily" => {
                self.repetition = Repetition::Daily;
            },
            "weeky" | "weekly" => {
                self.repetition = Repetition::Weekly;
            }
            "month" | "monthly" => {
                self.repetition = Repetition::Monthly;
            },
            "never" | "none" => {
                self.repetition = Repetition::Never;
            }
            &_ => {
                println!("Set Repetition: No such repetition: {}", rep);
            }

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

pub struct RawTaskitem {
    pub start: Vec<u32>,
    pub repetition: Repetition,
    pub title: String,
    pub note: String,
    pub completed: Vec<Vec<u32>>,
    pub finished: bool,
}
