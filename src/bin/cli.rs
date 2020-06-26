use std::io;
extern crate chrono;

extern crate to_do;
use to_do::async_direct_cmd::{AsyncCmd, Args};
use to_do::{task_item, TDError};
use to_do::parser_help::detailed_help;

type CmdResult<T> = std::result::Result<T, TDError>;

fn read(cmd_raw: &mut String) -> CmdResult<usize> {
    let len = io::stdin().read_line(cmd_raw)?;
    Ok(len)
}

extern crate dotenv;

fn run() -> Result<(), TDError> {

    let parser = task_item::CmdParser::new();
    let fallback_parser = task_item::RecoveryParser::new();
    let cmdline = AsyncCmd::new()?;

    loop {
        let mut cmd_raw = String::new();
        let _bytes_read = read(&mut cmd_raw)?;
        let cmd = parser.parse(&cmd_raw);

        let cmd_result = match cmd {
            Ok(Args::MakeRaw(raw)) => cmdline.make(raw),
            Ok(Args::List) => cmdline.list_all(),
            Ok(Args::Show(rep, when)) => cmdline.show(rep, when), // this needs to change so we can see period around [date]
            Ok(Args::Mods(id, mods)) => cmdline.modify(id, mods),
            Ok(Args::Detail(id)) => cmdline.detail(id),
            Ok(Args::Do(id, date, finished)) => cmdline.do_task(id, date, finished),
            Ok(Args::Help(cmd)) => Ok(detailed_help(cmd)),
            Ok(Args::Quit) => break,
            Err(_e) => {
                match fallback_parser.parse(&cmd_raw) {
                    Ok(cmd) => detailed_help(Some(cmd)),
                    Err(_) => detailed_help(None)
                };
                Ok(())
            },
            _ => Ok(())
        };

        // handles error inside the above
        match cmd_result {
            Err(e) => {
                println!("{}", e);
            },
            Ok(_r) => {}
        }
    }
    Ok(())
}

fn main () -> CmdResult<()> {
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

    run()?;
    Ok(())
}
