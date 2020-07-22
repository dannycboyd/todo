// CLI Tool version of calendar. Works with direct connection to postgres
// Rename this file, and respec the functions to be more generalized, and then we can put this as the backend to both the CLI and the server.
// Since the update to diesel none of this needs to be async
use super::old_task::{ RawTaskItem, Mod };
use super::{cal, TDError, establish_connection};

use diesel::PgConnection;
// use super::models::{Completion, NewCompletion, Task, NewTask, TaskUpdate};
use super::models::{task, task_completions};
use task::{Task, NewTask, TaskUpdate};
use task_completions::{Completion, NewCompletion};

#[derive(Debug)]
pub enum Args {
    Do(i32, Option<Vec<u32>>, bool),
    Help(Option<String>),
    List,
    MakeRaw(RawTaskItem),
    Mods(i32, Vec<Mod>),
    // Save,
    Show(cal::Repetition, Option<Vec<u32>>),
    Detail(i32),
    NoOp,
    Quit,
}

pub struct AsyncCmd {
    connection: PgConnection
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
    pub fn new() -> Result<Self, TDError> {
      Ok(Self { connection: establish_connection() })
    }

    /**
     * get all tasks and display them for a given chunk of time.
     */
    pub fn show(&self, kind: cal::Repetition, date_raw: Option<Vec<u32>>) -> Result<(), TDError> {
      use super::schema::tasks::dsl::*;
      use diesel::prelude::*;

      let rows = tasks.load::<Task>(&self.connection)?;
      let start_date = cal::date_or_today(date_raw);
      Ok(cal::show_type(kind, start_date, rows))
    }

    pub fn detail(&self, search_id: i32) -> Result<(), TDError> {
      use super::schema::tasks::dsl::*;
      use diesel::prelude::*;

      let found_task = tasks.filter(id.eq(search_id))
        .limit(1)
        .load::<Task>(&self.connection)?;
      let found_completions: Vec<Completion> = Completion::belonging_to(&found_task)
        .load(&self.connection)?;
      
      if found_task.len() > 0 {
        println!("{}", found_task[0]);
        for completion in found_completions {
          println!("{}", completion.get_date())
        }
      }
      Ok(())
    }

    pub fn list_all(&self) -> Result<(), TDError> {
      use super::schema::tasks::dsl::*;
      use diesel::prelude::*;

      let all_tasks = tasks.load::<Task>(&self.connection)?;

      for found_task in all_tasks {
        println!("{}\n", found_task.to_string())
      }
      Ok(())

    }

    pub fn make(&self, raw: RawTaskItem) -> Result<(), TDError> {
      use super::schema::tasks;
      use diesel::prelude::*;

      let start = cal::get_start(&raw.start)?;

      let new_task = NewTask {
        title: raw.title,
        start: start,
        repeats: raw.repetition,
        note: raw.note,
        finished: raw.finished,
      };
      let _inserted_task = diesel::insert_into(tasks::table)
        .values(&new_task)
        .get_result::<Task>(&self.connection)?; // get_result needs to know what returning type to use.
        println!("Inserted task:\n    {}", _inserted_task.to_string());
      Ok(())
    }

    pub fn modification_to_update(changes: Vec<Mod>) -> Result<TaskUpdate, TDError> {
      let mut update = TaskUpdate {
        start: None,
        repeats: None,
        title: None,
        note: None,
        finished: None
      };

      for change in changes {
        match change {
          Mod::Start(raw_date) => {
            let date = cal::get_start(&raw_date)?;
            update.start = Some(date);
          },
          Mod::Title(new_title) => { update.title = Some(new_title); },
          Mod::Note(new_note) => { update.note = Some(new_note); },
          Mod::Rep(new_repetition) => { update.repeats = Some(new_repetition.to_sql_string()); },
        }
      };
      Ok(update)
    }

    pub fn modify(&self, search_id: i32, changes: Vec<Mod>) -> Result<(), TDError> {

      use super::schema::tasks::dsl::*;
      use diesel::prelude::*;
      
      // get the list of changes

      if changes.len() > 0 {
        let update = AsyncCmd::modification_to_update(changes)?;

        let q = diesel::update(tasks.find(search_id)).set::<TaskUpdate>(update);
        println!("{:?}", diesel::debug_query::<diesel::pg::Pg, _>(&q));
        let task = q.get_result::<Task>(&self.connection)?;

        Ok(println!("Updated! {}", task.to_string()))
      } else {
        Err(TDError::NoneError)
      }
    }

    pub fn do_task(&self, search_id: i32, date: Option<Vec<u32>>, finished: bool) -> Result<(), TDError> {
      use super::schema::{task_completions};
      use diesel::prelude::*;

      let date = cal::date_or_today(date);
      let _inserted_task = diesel::insert_into(task_completions::table)
        .values(&NewCompletion::new(search_id, date))
        .get_result::<Completion>(&self.connection)?; // get_result needs to know what returning type to use.

      println!("Completed task {} for date {}", search_id, date);

      if finished {
        use super::schema::tasks::dsl::*;
        let task = diesel::update(tasks.find(search_id)).set(finished.eq(true)).get_result::<Task>(&self.connection)?;
        println!("Marked task {} as finished!\n{}", search_id, task.to_string());
      }
      Ok(())
    }
}
