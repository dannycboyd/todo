extern crate chrono;
// use chrono::prelude::*; // Utc, Local
use chrono::NaiveDate;
use crate::cal::calendar::Repetition;
// use crate::cal::calendar;
static mut NEXT_ID: u32 = 1;

#[derive(Debug)]
pub struct TaskItem {
    id: u32,
    pub start: NaiveDate,
    pub repetition: Repetition,
    pub title: String,
    pub note: String,
    // completed: [DateTime<Utc>], // Should use a vector here, maybe. Other solution?
    pub finished: bool,
}

impl TaskItem {
    pub unsafe fn new(start: NaiveDate) -> TaskItem {
        NEXT_ID += 1;
        TaskItem {
            id: NEXT_ID,
            start,
            repetition: Repetition::Weekly,
            title: String::from("Title"),
            note: String::from(""),
            finished: false,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title)
    }

    pub fn set_note(&mut self, note: &str) {
        self.note = String::from(note)
    }

    // pub fn occurs_on_day(&self, check: NaiveDate) -> bool {
    //     calendar::task_on_day(&self.start, &self.repetition, check)
    // }
}

//    impl TaskItem {
//        fn daysThisWeek(sunday: DateTime<Utc>)
//    }
