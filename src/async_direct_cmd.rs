// command line crate. Works on local storage .json. I want a similar API sort of thing like this but for the server.
use std::io::{Write};

// use serde_json;
// use std::fs::File;
// use std::fs;
use std::error;

use futures::{FutureExt};
use futures::stream::TryStreamExt;

use tokio_postgres::{NoTls};
use tokio_postgres;

use super::task::{ TaskItem, RawTaskItem, Mods };
use super::{cal, TDError, from_row};

#[derive(Debug)]
pub enum Args {
    Do(u32),
    Finish(u32),
    Help,
    List,
    MakeRaw(RawTaskItem),
    Mods(u32, Vec<Mods>),
    Save,
    Show(cal::Repetition, Option<Vec<u32>>),
    Quit,
}

pub struct AsyncCmd {
    // storage: Vec<TaskItem>,
    // connection: String,
    client: tokio_postgres::Client,
}

impl AsyncCmd {
    pub async fn new() -> Result<Self, TDError> {
        let (client, connection) = tokio_postgres::connect("host=localhost user=dannyboyd dbname=caldata", NoTls).await?;
        let connection = connection.map(|r| {
          if let Err(e) = r {
            eprintln!("Connection error: {}", e)
          }
        });
        tokio::spawn(connection);
        let me = Self { client: client };
        Ok(me)
    }

    // pub fn handle_load(&mut self, url: &str) {
    //     match self.load(url) {
    //         Ok(count) => println!("Loaded {} tasks {}", count, url),
    //         Err(e) => println!("Error occurred during file load: {:?}", e),
    //     }
    // }

    // fn load(&mut self, url: &str) -> Result<usize, Box<dyn error::Error>> {
    //     let file = fs::read_to_string(url)?;
    //     self.storage = serde_json::from_str(&file)?;
    //     let len = self.storage.len();
    //     if len > 0 {
    //         unsafe {
    //             TaskItem::set_id_start(self.storage[len - 1].get_id());
    //         }
    //     }
    //     Ok(len)
    // }

    // pub fn save(&self, url: &str) -> Result<(), TDError> {
    //     if self.storage.len() > 0 {
    //         let mut file = File::create(url)?;
    //         let contents = serde_json::to_string(&self.storage)?;
    //             // .or_else(|_e| { Err(TDError::SerializeError) })?; // 3 - couldn't serialize output: if this happens, serde is broken
    //         write!(file, "{}", contents)?;
    //         Ok(())
    //     } else {
    //         Err(TDError::IOError(String::from("No items to save"))) // 1 - empty
    //     }
    // }

    // pub fn show(&self, kind: cal::Repetition, date_raw: Option<Vec<u32>>) {
    //     let start = cal::date_or_today(date_raw);
    //     cal::show_type(kind, start, &self.storage);
    // }

    pub async fn list_all(&self) -> Result<(), TDError> {
      let stmt = self.client.prepare("SELECT * FROM tasks").await?;
      let rows = self.client
        .query(&stmt, &[])
        .await?;

        for row in rows {
          match from_row(row) {
            Ok(item) => {
              println!("{}\n", item);
            },
            Err(e) => {
              eprintln!("An error occurred: {}", e);
            }
          }
        }
        Ok(())

    }

    // fn find_task_by_id(&mut self, id: i32) -> Option<&mut TaskItem> {
    //     self.storage.iter_mut().find(|task| task.get_id() == id)
    // }

    // pub fn modify(&mut self, id: i32, cmds: Vec<Mods>) {
    //     match self.find_task_by_id(id) {
    //         Some(task) => {
    //             task.apply_modifications(cmds);
    //         },
    //         None => println!("No task exists with id {}!", id),
    //     }
    // }

    // pub fn do_task(&mut self, id: i32) {
    //     match self.find_task_by_id(id) {
    //         Some(task) => {
    //             println!("Mark done today");
    //             task.mark_completed(cal::date_or_today(None));
    //             println!("Done! {:?}", task)
    //         },
    //         None => println!("Can't find task with id {}", id),
    //     }
    // }

    // pub fn finish_task(&mut self, id: i32) {
    //     match self.find_task_by_id(id) {
    //         Some(task) => {
    //             println!("Mark finished today");
    //             task.mark_finished(cal::date_or_today(None));
    //             println!("Finished {}", task)
    //         },
    //         None => println!("Can't find task with id {}", id),
    //     }
    // }

    // pub fn make_raw(&mut self, raw: RawTaskItem) {
    //     unsafe {
    //         match TaskItem::from_raw(raw) {
    //             None => println!("An error occurred parsing the raw task item. Likely an issue with the dates"),
    //             Some(task) => {
    //                 println!("New task: {:?}", task);
    //                 self.storage.push(task);
    //             }
    //         }
    //     }
    // }
}
