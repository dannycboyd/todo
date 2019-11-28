use std::io;
extern crate chrono;

extern crate to_do;
use to_do::parser_cmd::{Cmd, Args};

use to_do::async_direct_cmd::{AsyncCmd};
use to_do::task_item;

use to_do::TDError;

type CmdResult<T> = std::result::Result<T, TDError>;

fn read(cmd_raw: &mut String) -> CmdResult<usize> {
    let len = io::stdin().read_line(cmd_raw)?;
    Ok(len)
}

fn parse(parser: &task_item::CmdParser, cmd_raw: &str) -> CmdResult<Args> {
    parser.parse(cmd_raw)
        .or_else(|err| {
            let foo = format!("{}", err);
            Err(TDError::ParseError(foo))
        })
}

async fn run2() -> Result<(), TDError> {
    let parser = task_item::CmdParser::new();
    let cmdline = AsyncCmd::new().await?;

    loop {
        let mut cmd_raw = String::new();
        let _bytes_read = read(&mut cmd_raw)?;
        let cmd = parse(&parser, &cmd_raw)?;

        let cmd_result = match cmd {
            Args::List => cmdline.list_all().await,
            Args::Quit => break,
            _ => Ok(())
        };

        match cmd_result {
            Err(e) => println!("{}", e),
            _ => ()
        };
    }
    Ok(())
}

#[tokio::main]
async fn main () -> Result<(), TDError> {
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

    run2().await?;
    Ok(())
}
