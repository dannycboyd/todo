extern crate chrono;
// use chrono::prelude::*; // Utc, Local
// use chrono::Date;


mod cmd;
use cmd::Cmd;

mod cal;

mod task;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub task_item);

const DEFAULT_FILE: &str = "./caldata.json";

fn main () {
    let parser = task_item::DateParser::new();
    println!("{:?}", parser.parse("22"));
    println!("{:?}", parser.parse("04-20"));
    println!("{:?}", parser.parse("06-29-'19"));
    println!("{:?}", parser.parse("02-01-2019"));
    let mut cmdline = Cmd {
        cmd: vec![],
        storage: vec![],
        cmd_raw: String::new(),
        // last: None,
    };
    cmdline.handle_load(DEFAULT_FILE);
    cmdline.exec();
}
