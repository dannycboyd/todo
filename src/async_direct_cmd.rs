// CLI Tool version of calendar. Works with direct connection to postgres

use futures::{FutureExt};

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
    pub async fn new(conn_info: &str) -> Result<Self, TDError> {
        let (client, connection) = tokio_postgres::connect(conn_info, NoTls).await?;
        let connection = connection.map(|r| {
          if let Err(e) = r {
            eprintln!("Connection error: {}", e)
          }
        });
        tokio::spawn(connection);
        let me = Self { client: client };
        Ok(me)
    }

    /**
     * how do I construct filters?
     * does constructing queries matter?
     * The only cases I really want are date filtering
     * I can do the date filtering in here.
     * How can I leverage date ranges to limit query sizes? Does limiting query sizes matter?
     * if I do like "show march 2019", I want only to query for dates where start <=march 1 2019 && (!finished && finished > april 1 2019)
     * I think the most useful thing now is to build a date range constructor.
     * 
     */

    async fn get_tasks(&self) -> Result<Vec<TaskItem>, TDError> {
      let stmt = self.client.prepare("SELECT * FROM tasks").await?;
      let rows = self.client
        .query(&stmt, &[])
        .await?;
      let mut ret: Vec<TaskItem> = vec![];
      for row in rows {
        match from_row(row) {
          Ok(item) => ret.push(item),
          Err(e) => {
            eprintln!("An error occurred: {}", e);
          }
        }
      }
      Ok(ret)
    }

    pub async fn show(&self, kind: cal::Repetition, date_raw: Option<Vec<u32>>) -> Result<(), TDError> {
      let rows = self.get_tasks().await?;
      let start = cal::date_or_today(date_raw);
      Ok(cal::show_type(kind, start, &rows))
    }

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
