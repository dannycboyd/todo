extern crate chrono;
use chrono::prelude::*; // Utc, Local
use chrono::Date;
use chrono::format::*; // parse

use std::io;
use std::convert;

mod task;
use task::*;

fn get_month() {
    let month = Local::now();
    println!("It is {}", month.format("%B, %Y"))
}

fn get_day(storage: &Vec<TaskItem>) {
    let now = Local::now().date();
    // let today = Local::now().day();
    println!("Today is {}", now.format("%a %B %e"));
    let mut found = false;
    if storage.len() > 0 {
        for i in 0..storage.len() {
            if storage[i].start == now {
                found = true;
                println!("{:?}", storage[i])
            }
        }
        if !found {
            println!("No tasks found today");
        }
    }
//    println!("Today is {:?}, the {} day of {}", now.weekday(), now.day(), now.month())

}

fn make_task_weekday(day: &str) {
    println!("Trying to make a task for {}", day)
}

fn make_parse(date: &str) -> Result<TaskItem, chrono::ParseError> {
    let date_only = NaiveDate::parse_from_str(date, "%Y-%m-%d");
    let error = false;
    match date_only {
        Ok(date) => {
            let start = Local.ymd(date.year(), date.month(), date.day());
            unsafe { Ok(TaskItem::new(start)) }
        },
        Err(e) => Err(e)
    }
}

fn make_task_monthday(day: u32) -> TaskItem {
    let today = Local::now().date();
    let start = Local.ymd(today.year(), today.month(), day);
    let task = unsafe { TaskItem::new(start) };
    //println!("today {:?}, start {:?}", today, start);
    println!("new task: {:?}", task);
    task
}

fn main() {
    let mut storage: Vec<TaskItem> = vec![];


    loop {

        println!("Please enter a command");
        let mut command = String::new();
        io::stdin().read_line(&mut command)
           .expect("Failed to read line");
        let cmd: Vec<&str> = command.trim().split(' ').collect();
        println!("{:?}", cmd);
        if cmd.len() >= 1 {
            match cmd[0] {
                "month" => get_month(),
                "today" => get_day(&storage),
                "make" => {
                    if cmd.len() >= 2 {
                        let day: u32 = String::from(cmd[1]).parse()
                            .expect("Usage: make <day>");
                        let task = make_task_monthday(day);
                        storage.push(task)
                    } else {
                        println!("Usage: make <day>")
                    }
                }
                "parse" => {
                if cmd.len() >= 2 {
                    let parsed_task = make_parse(cmd[1]);
                    match parsed_task {
                        Ok(task) => storage.push(task),
                        Err(e) => println!("{}", e),
                    }
                } else {
                    println!("Usage: \"parse <YYYY-MM-DD>\"")
                }
                }
                "break" | "quit" | "exit" => break,
                &_ => println!("Unknown command: \"{}\"", cmd[0]),
            }
        }
    }
}
