// CLI Tool version of calendar. Works with direct connection to postgres

use futures::{FutureExt};

use tokio_postgres::{NoTls};
use tokio_postgres;
use chrono::NaiveDate;

use super::task::{ TaskItem, RawTaskItem, Mods };
use super::{cal, TDError, from_row};

#[derive(Debug)]
pub enum Args {
    Do(i32, Option<Vec<u32>>, bool),
    Help(Option<String>),
    List,
    MakeRaw(RawTaskItem),
    Mods(i32, Vec<Mods>),
    // Save,
    Show(cal::Repetition, Option<Vec<u32>>),
    Detail(i32),
    NoOp,
    Quit,
}

pub struct AsyncCmd {
    // storage: Vec<TaskItem>,
    // connection: String,
    client: tokio_postgres::Client,
}

/**
 * Something which would be nice to have is step-by-step commands
 * make\n
 * > When? > dd-mm-yyyy\n
 * Title > blahblahblah\n
 * Notes?
 * etc
 */

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

    #[allow(dead_code)]
    async fn task_by_id(&self, id: i32) -> Result<TaskItem, TDError> {
      let filters = vec![(String::from("id"), String::from("="), format!("{}", id))];
      let tasks = self.get_tasks_by(filters).await?;
      match tasks.first() {
        Some(t) => Ok(t.clone()),
        None => Err(TDError::NoneError)
      }
    }

    #[allow(dead_code)]
    async fn get_tasks_by(&self, filters: Vec<(String, String, String)>) -> Result<Vec<TaskItem>, TDError> {
      let mut query = String::from("SELECT * FROM tasks ");
      for (index, filter) in filters.iter().enumerate() {

        let join = match index {
          0 => "where",
          _ => "and"
        };
        query.push_str(&format!("{} {} {} {}", join, filter.0, filter.1, filter.2))
      };


      let rows = self.client.query(query.as_str(), &[]).await?;
      let mut ret: Vec<TaskItem> = vec![];
      for row in rows {
        match from_row(row) {
          Ok(item) => ret.push(item),
          Err(e) => eprintln!("An error trying to parse task: {}", e)
        }
      }
      Ok(ret)
    }

    pub async fn show(&self, kind: cal::Repetition, date_raw: Option<Vec<u32>>) -> Result<(), TDError> {
      let rows = TaskItem::get_all(&self.client).await?;
      let start = cal::date_or_today(date_raw);
      Ok(cal::show_type(kind, start, &rows))
    }

    pub async fn detail(&self, id: i32) -> Result<(), TDError> {
      let task = TaskItem::get(&self.client, id).await?;
      println!("{}", task);

      let dates = TaskItem::get_completions(&self.client, id).await?;
      for date in dates {
        println!("{}", date)
      }
      Ok(())
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

    /* I don't like this. Make a function to get the sql args for a rawtaskitem */
    pub async fn make(&self, raw: RawTaskItem) -> Result<(), TDError> {
      raw.insert(&self.client).await?;
      Ok(())
    }

    pub async fn modify(&self, id: i32, changes: Vec<Mods>) -> Result<(), TDError> {
      if changes.len() > 0 {
        let mut query_str = String::from("UPDATE tasks SET ");
        for change in changes {
          let change = change.to_sql()?;
          query_str.push_str(&change);
        }

        query_str.push_str(&format!("WHERE id = {}", id));

        println!("{}", query_str);
        let _r = self.client.query(query_str.as_str(), &[]).await?;

        Ok(())
      } else {
        Err(TDError::NoneError)
      }
    }

    pub async fn do_task(&self, id: i32, date: Option<Vec<u32>>, finished: bool) -> Result<(), TDError> {
      let query_str = String::from("INSERT INTO task_completions (task_id, date) VALUES ($1, $2) RETURNING id");
      let date = cal::date_or_today(date);
      let r = self.client.query(query_str.as_str(), &[&id, &date]).await?;

      if r.len() > 0 {
        println!("Completed task {} for date {}", id, date);
      } else {
        return Err(TDError::PostgresError("Something went wrong".to_string()));
      }

      if finished {
        let query_str = String::from("UPDATE tasks SET finished=$1 where id = $2");
        let _r = self.client.query(query_str.as_str(), &[&finished, &id]).await?;
      }
      Ok(())
    }
}
