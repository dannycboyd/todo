extern crate chrono;
extern crate to_do;
use to_do::{TDError, establish_connection};

extern crate hyper;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{service_fn, make_service_fn};
use hyper::server::conn::AddrStream;

use url::Url;

use futures::stream::TryStreamExt;

// use dotenv::dotenv;
// use std::env;

use diesel::PgConnection;
use to_do::models::{task};
use task::{Task, NewTask};

async fn do_delete(connection: PgConnection, uri: &str) -> Result<String, TDError> {
    use to_do::schema::tasks::dsl::*;
    use diesel::prelude::*;
    
    println!("{}", uri);
    let params = Url::parse(uri)?;
    println!("{:?}", params);
    let params = params.query_pairs();

    let mut task_id: i32 = 0;
    for param in params {
        println!("{:?}", param);
        if param.0 == "id" {
            task_id = param.1.to_string().parse::<i32>()?;
            break;
        }
    }
    if task_id == 0 {
        return Err(TDError::NoneError);
    }
    
    diesel::delete(tasks.filter(id.eq(&task_id))).execute(&connection)?;
    Ok(String::from(format!("Successfully deleted task # {}", task_id)))
}

async fn delete_task(connection: PgConnection, uri: String) -> Response<Body> {
    match do_delete(connection, &uri).await {
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
// rename this pls
async fn myecho(req: Request<Body>) -> Result<Response<Body>, TDError> {
    let conn_info = establish_connection();

    let method = req.method();
    let path = req.uri().path();

    println!("{}: {:?}", method, req);

    match (method, path) {
        (&hyper::Method::GET, "/special") => {
            println!("this one is special :)");
            Ok(Response::new(Body::from("Special Response :)")))
        },
        (&hyper::Method::GET, "/all") => {
            use to_do::schema::tasks::dsl::*;
            use diesel::prelude::*;
            let mut resp = String::new();

      
            let rows = tasks.load::<Task>(&conn_info)?;
            // let start_date = cal::date_or_today(date_raw); // we want to add this in the future
            // update cal to take a output stream or something
            for row in rows {
                resp.push_str(row.to_string().as_ref());
                resp.push('\n');
            }
            Ok(Response::new(Body::from(resp)))
        },
        (&hyper::Method::POST, "/task") => {
            let entire_body = req.into_body().try_concat().await?;

            let body_str = String::from_utf8(entire_body.to_vec())?;
            let deserialized: Result<NewTask, serde_json::error::Error> = serde_json::from_str(&body_str);
            Ok(match deserialized {
                Ok(to_insert) => {
                    use to_do::schema::tasks;
                    use diesel::prelude::*;

                    let inserted_task = diesel::insert_into(tasks::table)
                        .values(&to_insert)
                        .get_result::<Task>(&conn_info)?;
                    println!("new task inserted: {}", inserted_task.to_string());
                    Response::new(Body::from(format!("Added new task: {}", inserted_task.to_string())))
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
            Ok(delete_task(conn_info, uri).await)
        }
        _ => {
            Ok(Response::new(Body::from(format!("{}, {}", method, path))))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), TDError> {
    // dotenv().ok();

    let portno: u16 = 8080;
    // Could still be useful in case we need to change the port, but I don't forsee a case for this at the current time.
    // match env::var_os("SERVICE_PORT") { // this was less useful than I thought, since we only care to change the outside port, not the internal one
    //     Some(num) => portno = str::parse(&num.into_string().unwrap()).unwrap(), // if this fails, it won't work anyway
    //     None => ()
    // }

    let addr = ([0, 0, 0, 0], portno).into();

    let make_svc = make_service_fn({
        move |socket: &AddrStream| {
            // let conn_info = conn_info.clone();
            let incoming_info = socket.remote_addr();
            async move {
                Ok::<_, TDError>(service_fn(move |r: Request<Body>| {
                    eprintln!("Incoming connection: {:?}: {} `{}`", incoming_info, r.method(), r.uri().path());
                    // let conn_info = conn_info.clone();
                    async move {
                        myecho(r).await
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
