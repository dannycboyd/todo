// CLI Tool version of calendar. Works with direct connection to postgres
// Rename this file, and respec the functions to be more generalized, and then we can put this as the backend to both the CLI and the server.
// Since the update to diesel none of this needs to be async
use super::old_task::Mods;
use super::{cal, TDError, establish_connection};
use cal::{date_or_today, show_type, Repetition};
use diesel::PgConnection;

use super::actions::item;
use super::models::item::{NewItem, Item, ItemFilter};

#[derive(Debug)]
pub enum Args {
  Do(i32, Option<Vec<u32>>, bool),
  Help(Option<String>),
  List,
  MakeRaw(NewItem),
  Mods(i32, Mods),
  // Save,
  Show(Repetition, Option<Vec<u32>>),
  Detail(i32),
  NoOp,
  Quit
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
    Ok(Self {
      connection: establish_connection()
    })
  }

  /**
   * get all tasks and display them for a given chunk of time.
   * simplify this
   */
  pub fn show(&self, kind: Repetition, date_raw: Option<Vec<u32>>) -> Result<(), TDError> {
    use super::schema::items::dsl::*;
    use diesel::prelude::*;

    let rows = items.load::<Item>(&self.connection)?;
    let start_date = date_or_today(date_raw);
    Ok(show_type(kind, start_date, rows))
  }

  // use actions here
  // pub fn detail(&self, search_id: i32) -> Result<(), TDError> {
  //   use super::schema::tasks::dsl::*;
  //   use diesel::prelude::*;

  //   let found_task = tasks
  //     .filter(id.eq(search_id))
  //     .limit(1)
  //     .load::<Task>(&self.connection)?;
  //   let found_completions: Vec<Completion> =
  //     Completion::belonging_to(&found_task).load(&self.connection)?;

  //   if found_task.len() > 0 {
  //     println!("{}", found_task[0]);
  //     for completion in found_completions {
  //       println!("{}", completion.get_date())
  //     }
  //   }
  //   Ok(())
  // }

  pub fn list_all(&self) -> Result<(), TDError> {
    let all_items = item::get_items(&self.connection, ItemFilter::new())?;

    for found_item in all_items {
      println!("{}\n", found_item.to_string());
    }
    Ok(())
  }

  pub fn upsert(&self, raw: NewItem) -> Result<(), TDError> {
    // println!("{:?}", raw);
    let item = item::upsert_item(raw, vec![], vec![], &self.connection)?;
    Ok(println!("Inserted new item:\n\t{}", item.to_string()))
  }

  pub fn modify(&self, search_id: i32, mods: Mods) -> Result<(), TDError> {
    // get the list of changes
    if mods.has_changes() {
      let mut update = NewItem::from(mods);
      update.id = Some(search_id);

      let item = item::upsert_item(update, vec![], vec![], &self.connection)?;
      Ok(println!("Modifications done! {}", item.to_string()))
    } else {
      Err(TDError::NoneError)
    }
  }

  // update this
  // pub fn do_task(
  //   &self,
  //   search_id: i32,
  //   date: Option<Vec<u32>>,
  //   finished: bool
  // ) -> Result<(), TDError> {
  //   use super::schema::{task_completions};
  //   use diesel::prelude::*;

  //   let date = cal::date_or_today(date);
  //   let _inserted_task = diesel::insert_into(task_completions::table)
  //     .values(&NewCompletion::new(search_id, date))
  //     .get_result::<Completion>(&self.connection)?; // get_result needs to know what returning type to use.

  //   println!("Completed task {} for date {}", search_id, date);

  //   if finished {
  //     use super::schema::tasks::dsl::*;
  //     let task = diesel::update(tasks.find(search_id))
  //       .set(finished.eq(true))
  //       .get_result::<Task>(&self.connection)?;
  //     println!(
  //       "Marked task {} as finished!\n{}",
  //       search_id,
  //       task.to_string()
  //     );
  //   }
  //   Ok(())
  // }
}
