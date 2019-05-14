extern crate chrono;
// use chrono::prelude::*; // Utc, Local
// use chrono::Date;


// mod cmd;
// use cmd::Cmd;

mod parser_cmd;
use parser_cmd::Cmd;

mod cal;

mod task;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub task_item);

const DEFAULT_FILE: &str = "./caldata.json";

fn main () {
    let date_p = task_item::DateParser::new();
    let rep_p = task_item::RepeatsParser::new();

    println!("{:?}", date_p.parse("22"));
    println!("{:?}", date_p.parse("04-20"));
    // println!("{:?}", date_p.parse("06-29-'19"));
    println!("{:?}", date_p.parse("02-01-2019"));

    println!("{:?}", rep_p.parse("n"));
    println!("{:?}", rep_p.parse("never"));
    println!("{:?}", rep_p.parse("d"));
    println!("{:?}", rep_p.parse("m"));

    let mut cmdline = Cmd::new();
    // cmdline.handle_load(DEFAULT_FILE);
    cmdline.exec();
}
