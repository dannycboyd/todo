extern crate chrono;
// use serde::{Serialize, Deserialize};
// use chrono::NaiveDate;
// use crate::cal::{Repetition};
use crate::cal;
// use std::fmt;
use crate::TDError;
// static mut NEXT_ID: i32 = 1;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct TaskItem {
//     id: i32,
//     pub start: NaiveDate,
//     pub repetition: Repetition,
//     pub title: String,
//     pub note: String,
//     pub completed: Vec<NaiveDate>,
//     pub finished: bool,
// }

#[derive(Debug)]
pub enum Mod {
    Start(Vec<u32>),
    Rep(cal::Repetition),
    Title(String),
    Note(String),
}

impl Mod {
    pub fn to_sql(&self) -> Result<String, TDError> {
        Ok(match self {
            Self::Start(date_raw) => {
                let start = cal::get_start(&date_raw.to_vec())?;
                let start = start.format("%Y-%m-%d").to_string();
                format!("start='{}' ", start)
            },
            Self::Rep(r) => format!("repeats='{}' ", r.to_sql_string()),
            Self::Title(t) => format!("title='{}' ", t),
            Self::Note(n) => format!("note='{}' ", n)
        })
    }
}

/**
 * I think _most_ of this class can go away. The modification functions need to be centralized here instead of in the server/async_cmd_direct
 * 
 * On the same note, most all of the update/creation happens by either raw task items OR by a vec<mods>, so it may be enough to just have a module for all this
 * instead of the TaskItem struct/impl, since we don't need to address a memory object, we just need to open a db connection and query it.
 * 
 * OTOH, we do want a way to print the results, so keeping this class so that we can pull stuff from the DB and then print it out is good.
 *
 * Update 6/13 I'm not sure how it will change
 */
// impl TaskItem {
//     pub unsafe fn new(start: NaiveDate, title: String, note: String, rep: Repetition) -> TaskItem {
//         NEXT_ID += 1;
//         TaskItem {
//             id: NEXT_ID,
//             start,
//             repetition: rep,
//             title,
//             note,
//             finished: false,
//             completed: vec![],
//         }
//     }
//     pub unsafe fn set_id_start(highest: i32) {
//         NEXT_ID = highest;
//     }

//     pub fn new_by_id(id: i32, start: NaiveDate, title: String, note: String, rep: Repetition, finished: bool) -> TaskItem {
//         TaskItem {
//             id,
//             start,
//             repetition: rep,
//             title,
//             note,
//             finished,
//             completed: vec![],
//         }
//     }

//     // pub unsafe fn from_raw(raw: RawTaskItem) -> Option<TaskItem> {
//     //     let start = cal::get_start(raw.start)?;
//     //     let rep = cal::Repetition::from_str(&raw.repetition)?;
//     //     let task = TaskItem::new(start, raw.title, raw.note, rep);
//     //     Some(task)
//     // }

//     pub fn apply_modifications(&mut self, mods: Vec<Mod>) {
//         for m in mods {
//             match m {
//                 Mod::Start(raw_start) => match cal::get_start(&raw_start.to_vec()) {
//                     Ok(start) => self.start = start,
//                     Err(_) => println!("Can't make {:?} into a date!", raw_start),
//                 },
//                 Mod::Rep(rep) => self.repetition = rep,
//                 Mod::Title(title) => self.title = title,
//                 Mod::Note(note) => self.note = note,
//             }
//         }
//     }

//     pub fn get_id(&self) -> i32 {
//         self.id
//     }

//     pub fn set_id(&mut self, new_id: i32) {
//         self.id = new_id;
//     }

//     // TODO: DELETE
//     pub fn mark_completed(&mut self, day: NaiveDate) {
//         let mut i: usize = 0;
//         while i < self.completed.len() {
//             let curr = self.completed[i];
//             if curr == day {
//                 println!("Already done on {}", day);
//                 return;
//             } else if curr < day {
//                 self.completed.insert(i, day);
//                 println!("Inserted at index {}", i);
//                 return;
//             }
//             i += 1;
//         }
//         println!("put it at the end");
//         self.completed.push(day)
//     }

//     pub fn mark_finished(&mut self, day: NaiveDate) {
//         self.mark_completed(day);
//         self.finished = true;
//     }

//     pub fn done_on_day(&self, day: NaiveDate) -> bool {
//         match self.completed.iter().find(|d| *d == &day) {
//             Some(_d) => true,
//             None => false,
//         }
//     }
//     // END

//     // Is there a cleaner way to do this?
//     // pub async fn get(client: &tokio_postgres::Client, id: i32) -> Result<TaskItem, TDError> {
//     //     let stmt = client.prepare("SELECT * FROM tasks WHERE id = $1").await?;
//     //     let rows = client.query(&stmt, &[&id]).await?;

//     //     for row in rows {
//     //         return from_row(row)
//     //     }
//     //     Err(TDError::NoneError)
//     // }

//     // pub async fn get_all(client: &tokio_postgres::Client) -> Result<Vec<TaskItem>, TDError> {
//     //     let stmt = client.prepare("SELECT * FROM tasks").await?;
//     //     let rows = client.query(&stmt, &[]).await?;
//     //     let mut tasks = vec![];

//     //     for row in rows {
//     //         match from_row(row) {
//     //             Ok(item) => tasks.push(item),
//     //             Err(e) => eprintln!("Could not parse task: {}", e)
//     //         }
//     //     }
//     //     Ok(tasks)
//     // }

//     // pub async fn get_completions(client: &tokio_postgres::Client, id: i32) -> Result<Vec<NaiveDate>, TDError> {
//     //     let stmt = client.prepare("SELECT * FROM task_completions WHERE task_id = $1").await?;
//     //     let rows = client.query(&stmt, &[&id]).await?;

//     //     let mut dates = vec![];
//     //     for row in rows {
//     //         let date: NaiveDate = row.try_get("date")?;
//     //         dates.push(date);
//     //     }
//     //     Ok(dates)
//     // }
// }

// impl super::TaskLike for TaskItem {
//     fn get_date(&self) -> NaiveDate {
//         self.start
//     }

//     fn get_rep(&self) -> Repetition {
//         self.repetition
//     }

//     fn is_finished(&self) -> bool {
//         self.finished
//     }

//     fn get_last_completed(&self) -> Option<&NaiveDate> {
//         self.completed.last()
//     }

//     fn to_string(&self) -> String {
//         String::from(format!("{}", self))
//     }
// }

// impl fmt::Display for TaskItem {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{} - {}: {}, {rep}\nNotes: {note}\nFinished: {finished}",
//             id=self.get_id(),
//             title=self.title,
//             start=self.start,
//             rep=self.repetition,
//             note=self.note,
//             finished=self.finished)
//     }
// }

#[derive(Debug)]
pub struct RawTaskItem {
    pub start: Vec<u32>,
    pub repetition: String,
    pub title: String,
    pub note: String,
    pub finished: bool,
}

impl RawTaskItem {
    pub fn new_empty() -> RawTaskItem {
        RawTaskItem {
            start: vec![],
            repetition: String::from("m"),
            title: String::from("Title"),
            note: String::from(""),
            finished: false,
        }
    }

    // pub async fn insert(&self, client: &tokio_postgres::Client) -> Result<(), TDError> {
    //     let query_str = String::from("INSERT INTO tasks (start, repeats, title, note, finished) VALUES ($1, $2, $3, $4, $5)");
    //     let stmt = client.prepare(&query_str).await?;
    //     let start = cal::get_start(&self.start)?;
    //     client.query(&stmt, &[&start, &self.repetition, &self.title, &self.note, &self.finished]).await?;
    //     Ok(())
    // }
}
