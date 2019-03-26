// command line crate
use std::io;

use crate::task::TaskItem;
use crate::cal::calendar;

pub struct Cmd {
    pub cmd: Vec<String>,
    pub storage: Vec<TaskItem>
}

impl Cmd {
    fn parse_date(&self, to_parse: &[String]) -> Vec<u32> {
        let mut values: Vec<u32> = vec![];
        for i in 0..to_parse.len() {
            match to_parse[i].parse() {
                Ok(val) => values.push(val),
                Err(e) => {
                    println!("Error: {} can't be parsed: {:?}", to_parse[i], e);
                    break;
                }
            }
        };
        values
    }

    fn make_task_from_values(&mut self, values: Vec<u32>) {
        // println!("{:?}, length {}", values, values.len());
        match calendar::get_start(values) {
            None => println!("Error: values entered can't be handled by calendar::get_start()"),
            Some(start) => {
                println!("{:?}", start);
                let task = unsafe { TaskItem::new(start) };
                println!("New task: {:?}", task);
                self.storage.push(task);
            }
        }
    }

    fn make_task(&mut self) {
        if self.cmd.len() > 1 {
            let mut values: Vec<u32> = vec![];
            for i in 1..self.cmd.len() {
                match self.cmd[i].parse() {
                    Ok(val) => values.push(val),
                    Err(e) => {
                        println!("Error: {} can't be parsed: {:?}", self.cmd[i], e);
                        break;
                    }
                }
            }
            match calendar::get_start(values) {
                None => println!("Error: values entered can't be handled by calendar::get_start()"),
                Some(start) => {
                    let task = unsafe { TaskItem::new(start) };
                    println!("new task: {:?}", task);
                    self.storage.push(task);
                }
            }
        } else {
            println!("usage: make <day> | <month> <day> [year]")
        }
    }

    // fn modify_task(&mut self) {
    //     let id = self.cmd[2].parse()?;
    //     let mut task: TaskItem;
    //     for i in 1..self.storage.len() {
    //         if (self.storage[i].get_id() == id) {
    //             match self.cmd[3] {
    //                 "title" => self.storage[i].set_title(self.cmd[4]),
    //                 "note" => self.storage[i].set_note(self.cmd[4]),
    //                 &_ => println!("Unknown property {}", self.cmd[3])
    //             };
    //             break;
    //         }
    //     }
    // }

    pub fn parse(&mut self) {
        match self.cmd[0].as_ref() {
            "make" | "new" => {
                self.make_task()
            },
            // "month" => {
            //     if self.cmd.len() > 1 {
            //         match self.cmd[1].parse() {
            //             Ok(month) => calendar::print_month(month),
            //             Err(e) => println!("An error occurred: {:?}", e)
            //         }
            //     }
            // },
            "month" => {
                if self.cmd.len() > 1 {
                    let max = self.cmd.len();
                    let max = if max > 4 {
                        4
                    } else {
                        max
                    };
                    match calendar::get_start(self.parse_date(&self.cmd[1..max])) {
                        Some(date) => calendar::print_month(date),
                        None => ()
                    }
                }
            }
            "make_parse" => {
                if self.cmd.len() > 1 {
                    let max = self.cmd.len();
                    let max = if max > 4 {
                        4
                    } else {
                        max
                    };
                    self.make_task_from_values(self.parse_date(&self.cmd[1..max]));
                }
            }
            // "modify" => {
            //     if (self.cmd.len() > 3) {
            //         match self.modify_task();
            //     }
            // }
            "help" | "h" => {
                // print out each command here
            }
            &_ => println!("Unknown command: {}", self.cmd[0])
        }
    }

    pub fn exec(&mut self) {
        loop {
            let mut args = String::new();
            io::stdin().read_line(&mut args)
               .expect("Failed to read line");
            self.cmd.clear();
            let args: Vec<&str> = args.trim().split(' ').collect();
            if args.len() > 0 {
                for arg in &args {
                    self.cmd.push(arg.to_string());
                }
                self.parse();
            }
        }
    }
}

// extern crate chrono;
// use chrono::prelude::*; // Utc, Local
// use chrono::Date;
// use chrono::format::*; // parse
//
// use std::convert;
//
// mod task;
// use task::*;
//
// fn get_month() {
//     let month = Local::now();
//     println!("It is {}", month.format("%B, %Y"))
// }
//
// fn get_day(storage: &Vec<TaskItem>) {
//     let now = Local::now().date();
//     // let today = Local::now().day();
//     println!("Today is {}", now.format("%a %B %e"));
//     let mut found = false;
//     if storage.len() > 0 {
//         for i in 0..storage.len() {
//             if storage[i].start == now {
//                 found = true;
//                 println!("{:?}", storage[i])
//             }
//         }
//         if !found {
//             println!("No tasks found today");
//         }
//     }
// //    println!("Today is {:?}, the {} day of {}", now.weekday(), now.day(), now.month())
//
// }
//
// fn show_id(cmd: Vec<&str>, storage: &Vec<TaskItem>) {
//     if cmd.len() >= 2 {
//         let search_id: u32 = String::from(cmd[1]).parse()
//             .expect("Usage: show <task_id>");
//         if storage.len() > 0 {
//             let mut found = false;
//             for i in 0..storage.len() {
//                 if storage[i].get_id() == search_id {
//                     found = true;
//                     println!("Task #{}: {:?}", search_id, storage[i]);
//                 }
//             }
//             if !found {
//                 println!("No tasks found for id {}", search_id)
//             }
//         }
//     } else {
//         println!("Usage: show <task_id>")
//     }
// }
//
// fn make_task_weekday(day: &str) {
//     println!("Trying to make a task for {}", day)
// }
//
// fn make_parse(date: &str) -> Result<TaskItem, chrono::ParseError> {
//     let date_only = NaiveDate::parse_from_str(date, "%Y-%m-%d");
//     // https://docs.rs/chrono/0.4.0/chrono/offset/trait.TimeZone.html#method.from_local_date
//     // once I'm snart I might be able to use this instead of the dumb way I've done it here
//     let error = false;
//     match date_only {
//         Ok(date) => {
//             let start = Local.ymd(date.year(), date.month(), date.day());
//             unsafe { Ok(TaskItem::new(start)) }
//         },
//         Err(e) => Err(e)
//     }
// }
//
// fn make_task_monthday(day: u32) -> TaskItem {
//     let today = Local::now().date();
//     let start = Local.ymd(today.year(), today.month(), day);
//     let task = unsafe { TaskItem::new(start) };
//     //println!("today {:?}, start {:?}", today, start);
//     println!("new task: {:?}", task);
//     task
// }
//
// fn parse_modify(cmd: &[&str]) {
//     // Pick a task, then for each field, update task
//     // [0] is the id
//     // [1-2] is the first pair
//     // [3-4] would be the next, etc
//     // carry index forward
//     if cmd.len() >= 3 {
//         let id: u32 = String::from(cmd[0]).parse()
//             .expect(&format!("Unable to parse task id: {}", cmd[0]));
//         for i in (1..cmd.len()).step_by(2) {
//             if (i + 1 < cmd.len()) {
//                 let field = cmd[i];
//                 let value = cmd[i+1];
//                 modify_task(id, field, value)
//             }
//         }
//     } else {
//         println!("usage: modify <id> <field> <value>");
//     }
// }
//
// fn modify_task(id: u32, field: &str, value: &str) {
//     println!("modify {}: {}<-{}", id, field, value)
// }
//
// fn main() {
//     let mut storage: Vec<TaskItem> = vec![];
//
//
//     loop {
//
//         println!("Please enter a command");
//         let mut command = String::new();
//         io::stdin().read_line(&mut command)
//            .expect("Failed to read line");
//         let cmd: Vec<&str> = command.trim().split(' ').collect();
//         // println!("{:?}", cmd);
//         if cmd.len() >= 1 {
//             match cmd[0] {
//                 "month" => get_month(),
//                 "today" => get_day(&storage),
//
//                 "show"   => show_id(cmd, &storage),
//
//                 "make" => {
//                     if cmd.len() >= 2 {
//                         let day: u32 = String::from(cmd[1]).parse()
//                             .expect("Usage: make <day>");
//                         let task = make_task_monthday(day);
//                         storage.push(task)
//                     } else {
//                         println!("Usage: make <day>")
//                     }
//                 },
//                 "parse" => {
//                     if cmd.len() >= 2 {
//                         let parsed_task = make_parse(cmd[1]);
//                         match parsed_task {
//                             Ok(task) => storage.push(task),
//                             Err(e) => println!("{}", e),
//                         }
//                     } else {
//                         println!("Usage: \"parse <YYYY-MM-DD>\"")
//                     }
//                 },
//                 "modify" => {
//                     if cmd.len() >= 3 {
//                         //println!("modify: {:?}", &cmd[1..])
//                         parse_modify(&cmd[1..]);
//                     } else {
//                         println!("Usage: modify <key> <value>")
//                     }
//                 }
//                 "break" | "quit" | "exit" => break,
//                 &_ => println!("Unknown command: \"{}\"", cmd[0]),
//             }
//         }
//     }
// }
