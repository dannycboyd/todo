extern crate chrono;
extern crate to_do;
use to_do::parser_cmd::Cmd;
use to_do::task::TaskItem;
use chrono::NaiveDate;
use to_do::cal::Repetition;

// extern crate hyper;
// use hyper::{Body, Request, Response, Server};
// use hyper::rt::Future;
// use hyper::service::service_fn;
// use hyper::{Method, StatusCode};

use futures::FutureExt;
use tokio_postgres::{NoTls, Error, Row};

// type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn fromRow(row: Row) -> TaskItem {
    let id: i32 = row.get(0);
    let date: NaiveDate = row.get("start");
    let rep: &str = row.get("repeats");
    let title: &str = row.get("title");
    let note: &str = row.get("note");
    let finished: bool = row.get("finished");
    TaskItem::new_by_id(id as u32, date, title.to_string(), note.to_string(), Repetition::Daily)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (mut client, connection) =
        tokio_postgres::connect("host=localhost user=dannyboyd dbname=caldata", NoTls).await?;

    let connection = connection.map(|r| {
        if let Err(e) = r {
            eprintln!("connection error: {}", e);
        }
    });
    tokio::spawn(connection);

    let stmt = client.prepare("SELECT * from tasks").await?;

    let rows = client
        .query(&stmt, &[])
        .await?;

    for row in rows {
        println!("{:?}", fromRow(row));
    }

    println!("help");

    Ok(())
}
