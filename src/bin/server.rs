extern crate chrono;
extern crate to_do;
use to_do::task::TaskItem;
use chrono::NaiveDate;
use to_do::cal::Repetition;
use to_do::TDError;

extern crate hyper;
use hyper::{Body, Request, Response, Server};
use hyper::service::{service_fn, make_service_fn};

use futures::{FutureExt};
use futures::stream::TryStreamExt;

use tokio_postgres::{NoTls, Row};
use tokio_postgres;

fn from_row(row: Row) -> Result<TaskItem, TDError> {
    let id: i32 = row.get(0);
    let date: NaiveDate = row.get("start");
    let rep: &str = row.get("repeats");
    let rep = rep.parse::<Repetition>()?;
    let title: &str = row.get("title");
    let note: &str = row.get("note");
    let finished: bool = row.get("finished");
    Ok(TaskItem::new_by_id(id as u32, date, title.to_string(), note.to_string(), rep, finished))
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
                match from_row(row) {
                    Ok(item) => {
                        resp.push_str(format!("{}", item).as_ref());
                        resp.push('\n');
                    },
                    Err(e) => {
                        eprintln!("An error occurred loading from DB: {}", e);
                    }
                }
                // println!("{:?}", from_row(row));
            }
            Ok(Response::new(Body::from(resp)))

        },
        (&hyper::Method::POST, "/test") => {
            let response = Response::new(Body::from("hi"));
            let entire_body = req.into_body().try_concat().await?;

            let body_str = String::from_utf8(entire_body.to_vec())?;
            let mut data: serde_json::Value = serde_json::from_str(&body_str)?;
            let d: Result<TaskItem, serde_json::error::Error> = serde_json::from_str(&body_str);
            match (d) {
                Ok(v) => println!("{}", v),
                Err(e) => eprintln!("{}", e), // this should return in the body with an error code :)
            }
            // data["test"] = serde_json::Value::from("test_value");

            // println!("{}", data);

            Ok(response)

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
