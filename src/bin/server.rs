extern crate chrono;
extern crate to_do;
use to_do::parser_cmd::Cmd;
use to_do::task::TaskItem;
use chrono::NaiveDate;
use to_do::cal::Repetition;
use to_do::TDError;

extern crate hyper;
use hyper::{Body, Request, Response, Server};
use hyper::service::{service_fn, make_service_fn};
use hyper::{Method, StatusCode};

use futures::{future, FutureExt};
use hyper::rt::Future;
use tokio_postgres::{NoTls, Row};
use tokio_postgres;

// type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn from_row(row: Row) -> TaskItem {
    let id: i32 = row.get(0);
    let date: NaiveDate = row.get("start");
    let rep: &str = row.get("repeats");
    let title: &str = row.get("title");
    let note: &str = row.get("note");
    let finished: bool = row.get("finished");
    TaskItem::new_by_id(id as u32, date, title.to_string(), note.to_string(), Repetition::Daily)
}

// you can wrap it in a function that uses your own error type, and then map_err()
async fn myecho(req: Request<Body>) -> Result<Response<Body>, TDError> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=dannyboyd dbname=caldata", NoTls).await?;

    let connection = connection.map(|r| {
        if let Err(e) = r {
            eprintln!("connection error: {}", e);
        }
    });

    tokio::spawn(connection);

    let method = req.method();
    let path = req.uri().path();

    println!("{}: {}", method, path);

    match (method, path) {
        (&hyper::Method::GET, "/special") => {
            println!("this one is special :)");
            Ok(Response::new(Body::from("Special Response :)")))
        },
        (&hyper::Method::GET, "/all") => {
            let stmt = client.prepare("SELECT * from tasks").await?;

            let rows = client
                .query(&stmt, &[])
                .await?;

            let mut resp = String::new();
            for row in rows {
                resp.push_str(format!("{}", from_row(row)).as_ref());
                resp.push('\n');
                // println!("{:?}", from_row(row));
            }
            Ok(Response::new(Body::from(resp)))

        },
        _ => {
            Ok(Response::new(Body::from(format!("{}, {}", method, path))))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {

    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(move |_| {
        async {

            Ok::<_, hyper::Error>(service_fn(myecho))
        }
    });

    let server = Server::bind(&addr)
        .serve(service);
    
    println!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
