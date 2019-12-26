use std::io;
extern crate chrono;

extern crate to_do;
use to_do::async_direct_cmd::{AsyncCmd, Args};
use to_do::{task_item, TDError, connection_info};
use to_do::parser_help::detailed_help;

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

async fn run() -> Result<(), TDError> {

    let db_string = connection_info()?;
    println!("{:?}", db_string);

    let parser = task_item::CmdParser::new();
    let fallbackParser = task_item::RecoveryParser::new();
    let cmdline = AsyncCmd::new(&db_string).await?;

    loop {
        let mut cmd_raw = String::new();
        let _bytes_read = read(&mut cmd_raw)?;
        let cmd = parser.parse(&cmd_raw);

        let cmd_result = match cmd {
            Ok(Args::MakeRaw(raw)) => cmdline.make(raw).await,
            Ok(Args::List) => cmdline.list_all().await,
            Ok(Args::Show(rep, when)) => cmdline.show(rep, when).await, // this needs to change so we can see period around [date]
            Ok(Args::Mods(id, mods)) => cmdline.modify(id, mods).await,
            Ok(Args::Detail(id)) => cmdline.show_id(id).await,
            Ok(Args::Do(id, date, finished)) => cmdline.do_task(id, date, finished).await,
            Ok(Args::Help(cmd)) => detailed_help(cmd),
            Ok(Args::Quit) => break,
            Err(e) => {
                match fallbackParser.parse(&cmd_raw) {
                    Ok(cmd) => detailed_help(Some(cmd)),
                    Err(e) => detailed_help(None)
                };
                Ok(())
            },
            _ => Ok(())
        };
    }
    Ok(())
}

#[tokio::main]
async fn main () -> CmdResult<()> {
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

    run().await?;
    Ok(())
}
