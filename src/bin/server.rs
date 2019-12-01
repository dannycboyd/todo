#![feature(async_closure)]

extern crate chrono;
extern crate to_do;
use to_do::task::TaskItem;
use chrono::NaiveDate;
use to_do::cal::Repetition;
use to_do::{TDError, connection_info};

extern crate hyper;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{service_fn, make_service_fn};
use hyper::server::conn::AddrStream;

use url::Url;

use futures::{FutureExt};
use futures::stream::TryStreamExt;
// use futures::future::Future;

use tokio_postgres::{NoTls, Row};
use tokio_postgres;

fn from_row(row: Row) -> Result<TaskItem, TDError> {
    let id: i32 = row.try_get(0)?;
    let date: NaiveDate = row.try_get("start")?;
    let rep: &str = row.try_get("repeats")?;
    let rep = rep.parse::<Repetition>()?;
    let title: &str = row.try_get("title")?;
    let note: &str = match row.try_get("note") {
        Ok(n) => n,
        Err(_e) => ""
    };
    let finished: bool = row.try_get("finished")?;
    Ok(TaskItem::new_by_id(id, date, title.to_string(), note.to_string(), rep, finished))
}

async fn delete_task(client: tokio_postgres::Client, uri: String) -> Response<Body> {
    let do_delete = async || -> Result<String, TDError> {
        println!("{}", uri);
        let params = Url::parse(&uri)?;
        println!("{:?}", params);
        let params = params.query_pairs();
    
        let mut id: i32 = 0;
        for param in params {
            println!("{:?}", param);
            if param.0 == "id" {
                id = param.1.to_string().parse::<i32>()?;
                break;
            }
        }
        if id == 0 {
            return Err(TDError::NoneError);
        }
        // let foo = format!("DELETE FROM tasks WHERE id = {}", id);
        let stmt = client.prepare("DELETE FROM tasks WHERE id = $1").await?;
        client.query(&stmt, &[&id]).await?;

        Ok(String::from("Successfully deleted the task"))
    };
    match do_delete().await {
        Ok(response) => Response::new(Body::from(response)),
        Err(TDError::ParseError(v)) => {
            Response::builder()
                .status(StatusCode::from_u16(409).unwrap())
                .body(Body::from(v))
                .unwrap()
        },
        Err(TDError::NoneError) => {
            Response::builder()
                .status(StatusCode::from_u16(409).unwrap())
                .body(Body::from("Query params do not contain `id`"))
                .unwrap()
        },
        Err(e) => {
            Response::builder()
                .status(StatusCode::from_u16(409).unwrap())
                .body(Body::from(format!("Internal server error! {}", e)))
                .unwrap()
        }
    }
    
}

// you can wrap it in a function that uses your own error type, and then map_err()
// in order to get the string in here, I think we need a function which returns |req| -> result<response<body>>, which we can call with the string. we don't need to call myecho, so it could concievably be turned into a closure ok
async fn myecho(req: Request<Body>, conn_info: String) -> Result<Response<Body>, TDError> {
    let (client, connection) =
        tokio_postgres::connect(&conn_info, NoTls).await?;

    let connection = connection.map(|r| {
        if let Err(e) = r {
            eprintln!("connection error: {}", e);
        }
    });
    tokio::spawn(connection);

    let method = req.method();
    let path = req.uri().path();

    println!("{}: {:?}", method, req);

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
            }
            Ok(Response::new(Body::from(resp)))

        },
        (&hyper::Method::POST, "/task") => {
            let entire_body = req.into_body().try_concat().await?;

            let body_str = String::from_utf8(entire_body.to_vec())?;
            let d: Result<TaskItem, serde_json::error::Error> = serde_json::from_str(&body_str);
            Ok(match d {
                Ok(mut v) => {
                    let stmt = client.prepare(
                        "INSERT into tasks (start, repeats, title, note, finished) VALUES ($1, $2, $3, $4, $5) RETURNING id"
                    ).await?;

                    let r = client
                        .query(&stmt, &[&v.start, &v.repetition.to_sql_string(), &v.title, &v.note, &v.finished])
                        .await?;
                    let id: i32 = r[0].get(0);
                    v.set_id(id);
                    println!("{}", v);
                    Response::new(Body::from(format!("Added new task: {}", v)))
                },
                Err(e) => {
                    eprintln!("{}", e);
                    let err = format!("Couldn't store todo task: {}", e);
                    Response::builder()
                        .status(StatusCode::from_u16(409).unwrap())
                        .body(Body::from(err))
                        .unwrap()
                }
            })
        },
        (&hyper::Method::DELETE, "/task") => {
            let host = req.headers().get("host").unwrap().to_str().unwrap(); // apparently the URI doesn't include the hostname, for some reason.
            let uri = format!("{}{}", host, req.uri().to_string()); // build the URI because we have to in order to parse the query params out.
            Ok(delete_task(client, uri).await)
        }
        _ => {
            Ok(Response::new(Body::from(format!("{}, {}", method, path))))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), TDError> {

    let addr = ([127, 0, 0, 1], 3000).into();
    let conn_info = connection_info()?;

    let make_svc = make_service_fn({
        let conn_info = conn_info.clone();
        move |socket: &AddrStream| {
            let conn_info = conn_info.clone();
            let incoming_info = socket.remote_addr();
            async move {
                Ok::<_, TDError>(service_fn(move |r: Request<Body>| {
                    eprintln!("Incoming connection: {:?}: {} `{}`", incoming_info, r.method(), r.uri().path());
                    let conn_info = conn_info.clone();
                    async move {
                        myecho(r, conn_info).await
                    }
                }))
            }
        }
    });

    let server = Server::bind(&addr)
        .serve(make_svc);
    
    println!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
