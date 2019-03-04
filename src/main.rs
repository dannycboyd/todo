extern crate chrono;
use chrono::prelude::*; // Utc, Local
use chrono::Date;

use std::io;
use std::convert;

mod task;
use task::*;

fn get_month() {
    let month = Local::now();
    println!("It is {}", month.format("%B, %Y"))
}

fn get_day() {
    let now = Local::now();
    // let today = Local::now().day();
    println!("Today is {}", now.format("%a %B %e"))
//    println!("Today is {:?}, the {} day of {}", now.weekday(), now.day(), now.month())
}

fn make_task_weekday(day: &str) {
    println!("Trying to make a task for {}", day)
}

fn make_task_monthday(day: u32) {
    let today = Local::now().date();
    let start = Local.ymd(today.year(), today.month(), day);
    let task = TaskItem::new(start);
    println!("new task: {:?}", task);
}

fn main() {
    let mut task = TaskItem {
        start: Local.ymd(2019, 3, 3),
//        start: NaiveDate::from_ymd(2019, 3, 3),
        repetition: Repetition::Daily,
        title: String::from("Test Task"),
        note: String::from("Just a note"),
        // completed: [],
        finished: false,
    };

    println!("test task: {:?}", task);

    loop {

        println!("Please enter a command");
        let mut command = String::new();
        io::stdin().read_line(&mut command)
           .expect("Failed to read line");
        let cmd: Vec<&str> = command.trim().split(' ').collect();
        println!("{:?}", cmd);
        if (cmd.len() >= 1) {
            match cmd[0] {
                "month" => get_month(),
                "today" => get_day(),
                "make" => {
                    if (cmd.len() >= 2) {
                        let day: u32 = String::from(cmd[1]).parse()
                            .expect("Usage: make <day>");
                        make_task_monthday(day);
                    } else {
                        println!("Usage: make <day>")
                    }
                }
                "break" | "quit" | "exit" => break,
                &_ => println!("Unknown command: \"{}\"", &command),
            }
        }
    }
}
