use std::io;

extern crate chrono;

extern crate to_do;
use to_do::parser_cmd::{Cmd, Args};
use to_do::task_item;
use to_do::DEFAULT_FILE;

enum Error {
    ReadError,
    ParseError(String),
    Quit
}

type CmdResult<T> = std::result::Result<T, Error>;

fn run() {
    let parser = task_item::CmdParser::new();
    let mut cmdline = Cmd::new();
    loop {
        let mut cmd_raw = String::new();
        let something: CmdResult<()> = io::stdin().read_line(&mut cmd_raw)
            .or_else(|_| { Err(Error::ReadError) })
            .and_then(|_len| {
                parser.parse(&cmd_raw)
                    .or_else(|err| {
                        let foo = format!("{}", err);
                        Err(Error::ParseError(foo))
                    })
            })
            .and_then(|cmd| {
                 match cmd {
                    Args::MakeRaw(raw) => cmdline.make_raw(raw),
                    Args::Do(id) => cmdline.do_task(id),
                    Args::Finish(id) => cmdline.finish_task(id),
                    Args::Mods(id, cmds) => cmdline.modify(id, cmds),
                    Args::Show(kind, when) => cmdline.show(kind, when),
                    Args::List => cmdline.list_all(),
                    Args::Save => cmdline.handle_save(DEFAULT_FILE),
                    Args::Help => (),
                    Args::Quit => return Err(Error::Quit),
                };
                Ok(())
            });
        match something {
            Ok(_) => (),
            Err(Error::Quit) => break,
            Err(Error::ParseError(e)) => println!("Parser error: {}", e),
            Err(Error::ReadError) => println!("An error occurred reading input!"),
        }
    }
}

fn main () {
    let date_p = task_item::DateParser::new();
    let rep_p = task_item::RepeatsParser::new();
    let per_p = task_item::PeriodParser::new();

    println!("{:?}", date_p.parse("22"));
    println!("{:?}", date_p.parse("04-20"));
    println!("{:?}", date_p.parse("02-01-2019"));

    println!("{:?}", rep_p.parse("n"));
    println!("{:?}", rep_p.parse("never"));
    println!("{:?}", rep_p.parse("d"));
    println!("{:?}", rep_p.parse("m"));

    println!("{:?}", per_p.parse("m 04-20"));
    println!("{:?}", per_p.parse("d"));

    run();
}
