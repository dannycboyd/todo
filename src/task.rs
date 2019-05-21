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

    pub unsafe fn from_raw(raw: RawTaskItem) -> Option<TaskItem> {
        let start = calendar::get_start(raw.start)?;
        let mut task = TaskItem::new(start, raw.title, raw.note, raw.repetition);
        Some(task)
    }

    // pub fn apply_raw(raw: RawTaskItem, mut task: TaskItem) -> TaskItem { // I can't quite hold in my head what this should be doing.
    // // Given a set of optional values, I want (if it exists) to apply them to a task.
    // // this function will take a TaskItem and mutate its values. Don't return optional
    // // For the date, if it exists, try to create a new date. If that exists, apply it.
    // // in all other cases, apply them directly
    //     match raw.start {
    //         Some(start_raw) => {
    //             match calendar::get_start(start_raw) {
    //                 Some(start) => task.start = start,
    //                 None => ()
    //             }
    //         }
    //         None => ()
    //     };
    //     match raw.repetition {
    //         Some(rep) => task.rep = rep,
    //         None => ()
    //     };
    //     match raw.title {
    //         Some(title) => task.title = title,
    //         None => ()
    //     };
    //     match raw.note {
    //         Some(note) => task.note = note,
    //         None => ()
    //     };
    //     task;
    // }

// How does this go? The
    // pub fn apply_modifications(mods: Vec<Modification>) -> Option<TaskItem> {
    //     for m in mods.iter() {
    //         match m {
    //             // fields go here, with functions to handle
    //             Modification::Start(raw_start) => (),
    //             Modification::Repetition(rep) => (),
    //             Modification::Title(title) => (),
    //             Modification::Note(note) => (),
    //         }
    //     }
    //     // how the heck do we do this
    //     // Need some structure containing optionals, then check them all?
    //     // Currently there's only 4 modifiable fields (should be 5, for Finished. Completed won't be modifiable beyond set/unset )
    //     // Vec<Modifications>, where a modification can be a field and a value
    //
    // }

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

#[derive(Debug)]
pub struct RawTaskItem {
    pub start: Vec<u32>,
    pub repetition: Repetition,
    pub title: String,
    pub note: String,
    pub finished: bool,
}

// pub struct RawTaskItem {
//     pub start: Option<Vec<u32>>,
//     pub repetition: Option<Repetition>,
//     pub title: Option<String>,
//     pub note: Option<String>,
//     pub finished: Option<bool>,
// }

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

    // pub fn new_defaults() -> RawTaskItem {
    //     start: Some(vec![]),
    //     repetition: Some(Repetition::Weekly),
    //     title: Some(String::from("Title")),
    //     note: Some(String::from("")),
    //     finished: Some(false)
    // }
}
