extern crate chrono;

use std::io;

extern crate to_do;
use to_do::parser_cmd::Cmd;
use to_do::task_item;

fn run() {
    let mut cmdline = Cmd::new();
    loop {
        let mut cmd = String::new();
        match io::stdin().read_line(&mut cmd) {
            Err(e) => println!("An error occurred reading line: {:?}", e),
            Ok(_len) => cmdline.parse_cmd(&cmd)
        }
    }
}

fn main () {
    let date_p = task_item::DateParser::new();
    let rep_p = task_item::RepeatsParser::new();
    let per_p = task_item::PeriodParser::new();

    println!("{:?}", date_p.parse("22"));
    println!("{:?}", date_p.parse("04-20"));
    // println!("{:?}", date_p.parse("06-29-'19"));
    println!("{:?}", date_p.parse("02-01-2019"));

    println!("{:?}", rep_p.parse("n"));
    println!("{:?}", rep_p.parse("never"));
    println!("{:?}", rep_p.parse("d"));
    println!("{:?}", rep_p.parse("m"));

    println!("{:?}", per_p.parse("m 04-20"));
    println!("{:?}", per_p.parse("d"));


    run();
    // cmdline.handle_load(DEFAULT_FILE);
}
