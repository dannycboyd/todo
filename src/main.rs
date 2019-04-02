extern crate chrono;
// use chrono::prelude::*; // Utc, Local
// use chrono::Date;


mod cmd;
use cmd::Cmd;

mod cal;

mod task;

fn main () {
    let mut cmdline = Cmd {
        cmd: vec![],
        storage: vec![],
        cmd_raw: String::new(),
        // last: None,
    };
    cmdline.exec();
}
