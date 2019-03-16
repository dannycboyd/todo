extern crate chrono;
use chrono::prelude::*; // Utc, Local
use chrono::Date;
static mut NEXT_ID: u32 = 1;

#[derive(Debug)]
pub struct MultiDays {
    sun: bool,
    mon: bool,
    tue: bool,
    wed: bool,
    thur: bool,
    fri: bool,
    sat: bool,
}

#[derive(Debug)]
pub enum CustomRep {
    Weekly(MultiDays),
    EveryXDays(u32),
}

#[derive(Debug)]
pub enum Repetition {
    Daily,
    Weekly,
    Monthly,
    Custom(CustomRep),
}

#[derive(Debug)]
pub struct TaskItem {
    id: u32,
    pub start: Date<Local>,
    pub repetition: Repetition,
    pub title: String,
    pub note: String,
    // completed: [DateTime<Utc>], // Should use a vector here, maybe. Other solution?
    pub finished: bool,
}

impl TaskItem {
    pub unsafe fn new(start: Date<Local>) -> TaskItem {
        NEXT_ID += 1;
        TaskItem {
            id: NEXT_ID,
            start,
            repetition: Repetition::Daily,
            title: String::from("Title"),
            note: String::from(""),
            finished: false,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

//    impl TaskItem {
//        fn daysThisWeek(sunday: DateTime<Utc>)
//    }
