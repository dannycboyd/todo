extern crate chrono;
extern crate to_do;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, middleware};
use diesel::r2d2::{self, ConnectionManager};

use dotenv::dotenv;
use log::{info, debug};

#[macro_use]
extern crate diesel_migrations;
use diesel::PgConnection;

embed_migrations!();

pub fn run_migrations(conn: &PgConnection) {
  let _ = diesel_migrations::run_pending_migrations(&*conn);
}

#[actix_rt::main] // using a lot from https://github.com/actix/examples/tree/master/diesel/src
async fn main() -> Result<(), std::io::Error> {
  // std::env::set_var("RUST_LOG", "actix_web=info");
  dotenv().ok();
  env_logger::init();
  debug!("logging initialized");
  let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
  let manager = ConnectionManager::<PgConnection>::new(connspec);
  let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");

  run_migrations(&pool.get().unwrap());

  let mut _portno: u16 = 8080;
  // Could still be useful in case we need to change the port, but I don't forsee a case for this at the current time.
  match std::env::var_os("SERVICE_PORT") {
    // this was less useful than I thought, since we only care to change the outside port, not the internal one
    Some(num) => _portno = str::parse(&num.into_string().unwrap()).unwrap(), // if this fails, it won't work anyway
    None => ()
  }

  let addr = "localhost:8080"; // TODO: pull this from env
  println!("Listening on http://{}", addr);

  HttpServer::new(move || {
    App::new()
      .wrap(
        Cors::default()
          .allowed_origin("http://localhost:4200") // change this so it's from env
          .allowed_methods(vec!["OPTIONS", "GET", "POST", "DELETE"])
          .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
          .allowed_header(header::CONTENT_TYPE)
          .max_age(3600)
      )
      // set up DB pool to be used with web::Data<Pool> extractor
      .data(pool.clone())
      .wrap(middleware::Logger::new("%a %r"))
      .configure(to_do::routes::config)
  })
  .bind(&addr)?
  .run()
  .await
}
