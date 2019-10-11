extern crate chrono;
extern crate to_do;
use to_do::parser_cmd::Cmd;
use to_do::task_item;

extern crate hyper;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Method, StatusCode};

extern crate futures;
use futures::future;
const PHRASE: &str = "Hello, World!";

type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }

    }
    Box::new(future::ok(response))
}

fn main () {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn(echo)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}

fn hello_world(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(PHRASE))
}
