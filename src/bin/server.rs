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
use futures::Stream;

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
        (&Method::POST, "/echo/reverse") => {
            let reversed = req
                .into_body()
                .concat2()
                .map(move |chunk| {
                    let body = chunk.iter()
                        .rev()
                        .cloned()
                        .collect::<Vec<u8>>();
                    *response.body_mut() = Body::from(body);
                    response
                });
            return Box::new(reversed)
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
