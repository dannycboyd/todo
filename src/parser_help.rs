/**
 * Help file. Should be updated every time the task_item.lalrpop file is updated, if necessary
 */

use super::TDError;

fn date_parts() -> String {
  String::from("d | mm-dd | mm-dd-yyyy") // this is confusing/conflicts with rep_parts
}

fn bool_parts() -> String {
  String::from("true | false")
}

fn rep_parts() -> String {
  String::from("d | daily | m | monthly | y | yearly | n | never")
}

fn task_parts() -> String {
  let parts3 = String::from("text_data: `Title` `Note` | t:`Title` | n:`Note`");
  format!("\n\tstart_date: {date}\n\trepetition: {rep}\n\t{text}", date=date_parts(), rep=rep_parts(), text=parts3)
}

fn mod_help() -> String {
  format!("mod id [modification...]\nmodification:{}", task_parts())
}

fn show_help() -> String {
  format!("show period:\n\tperiod: {}", rep_parts())
}

fn detail() -> String {
  format!("detail id\n\tid: number")
}

fn do_help() -> String {
  format!("do id [on] [finished]\n\ton: {}\n\tfinished: {}", date_parts(), bool_parts())
}

fn list() -> String {
  format!("list\nShow all tasks")
}

pub fn basic_help() {
  println!("Valid commands are 'mod', 'show', 'detail', 'do', 'list', 'help', 'q'")
}

// may be worthwhile to make the recognized commands into an enum? they're written separately multiple places
pub fn detailed_help(cmd: Option<String>) -> Result<(), TDError> {
  match cmd {
    Some(cmd) => {
      match cmd.as_ref() { // fix this mess so it does a boilerplate "usage: {}", cmd_help() sort of thing
        "mod" => println!("{}\nModify a task by id.", mod_help()),
        "show" => println!("{}\nShow tasks in surrounding period.", show_help()),
        "detail" => println!("{}\nShow detailed info on task with id.", detail()),
        "do" => println!("{}\nMark a task as completed, with optional date and set as finished", do_help()),
        "list" => println!("{}", list()),
        "help" => println!("help [command]\nDisplay command info"),
        "q" => println!("q\nQuit program"),
        _ => println!("Unrecognized command: {}", cmd),
      }
    },
    None => basic_help()
  }
  Ok(())
}
