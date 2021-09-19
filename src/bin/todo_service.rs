extern crate chrono;
extern crate to_do;

use actix_cors::Cors;
use actix_web::{
  App, Error, HttpResponse, HttpServer, delete, get, http::header, middleware, post, web
};
use diesel::r2d2::{self, ConnectionManager};

use dotenv::dotenv;

#[macro_use]
extern crate diesel_migrations;
use diesel::PgConnection;
use to_do::{DbPool, get_pool_connection, parse_json};

use to_do::actions;
use to_do::models::{reference, responses, user};
use reference::NewItemRef;

embed_migrations!();

// break these out into modules (by path, maybe?)
// ^ ongoing effort into the /routes folder

#[get("/item/related/{item_id}")] // This is doing what the get item by id should be doing.
async fn get_related_by_id(
  pool: web::Data<DbPool>,
  path_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let item_id = path_id.into_inner();

  let item = web::block(move || actions::item::get_references_by_id(item_id, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })
    .unwrap();

  if let Some(items) = item {
    Ok(HttpResponse::Ok().json(items))
  } else {
    let res =
      HttpResponse::NotFound().body(format!("No related items were found for ID {}", item_id));
    Ok(res)
  }
}

#[post("/references/")]
async fn post_references(
  pool: web::Data<DbPool>,
  payload: web::Payload
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);

  let refs = parse_json::<Vec<NewItemRef>>(payload).await?;

  let refs = web::block(move || actions::item::insert_references(refs, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;

  Ok(HttpResponse::Ok().json(refs))
}

#[delete("/references")]
async fn delete_references(
  pool: web::Data<DbPool>,
  payload: web::Payload
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);

  let refs = parse_json::<Vec<NewItemRef>>(payload).await?;

  let refs = web::block(move || actions::item::delete_references(refs, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;
  Ok(HttpResponse::Ok().json(refs))
}

#[post("/user/new")]
async fn create_user(
  pool: web::Data<DbPool>,
  payload: web::Payload
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let mut request_body = parse_json::<user::NewUserRequest>(payload).await?;

  request_body.user.pwd_hash = None;
  request_body.user.pwd_salt = None;
  let new_user: user::User =
    web::block(move || actions::user::create_user(&conn, request_body.user, request_body.password))
      .await
      .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
      })?;
  let res = responses::Response {
    id: None,
    message: format!(
      "Successfully created new user {} {}",
      new_user.firstname, new_user.lastname
    ),
    value: None
  };
  Ok(HttpResponse::Ok().json(res))
}

#[get("/user/login")]
async fn login_user(
  pool: web::Data<DbPool>,
  query: web::Query<user::LoginRequest>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let user: user::LoginRequest = query.into_inner();

  let login = web::block(move || actions::user::login_user(user.id, user.password, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;

  Ok(HttpResponse::Ok().body(format!("Login {}", login)))
}

pub fn run_migrations(conn: &PgConnection) {
  let _ = diesel_migrations::run_pending_migrations(&*conn);
}

#[actix_rt::main] // using a lot from https://github.com/actix/examples/tree/master/diesel/src
async fn main() -> Result<(), std::io::Error> {
  std::env::set_var("RUST_LOG", "actix_web=error");
  env_logger::init();
  dotenv().ok();
  let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
  let manager = ConnectionManager::<PgConnection>::new(connspec);
  let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");

  run_migrations(&pool.get().unwrap());

  let _portno: u16 = 8080;
  // Could still be useful in case we need to change the port, but I don't forsee a case for this at the current time.
  // match env::var_os("SERVICE_PORT") { // this was less useful than I thought, since we only care to change the outside port, not the internal one
  //     Some(num) => portno = str::parse(&num.into_string().unwrap()).unwrap(), // if this fails, it won't work anyway
  //     None => ()
  // }

  let addr = "localhost:8080"; // TODO: pull this from env
  println!("Listening on http://{}", addr);

  HttpServer::new(move || {
    App::new()
      .wrap(
        Cors::default()
          .allowed_origin("http://localhost:4200")
          // .allowed_origin("*")
          // .send_wildcard()
          .allowed_methods(vec!["OPTIONS", "GET", "POST", "DELETE"])
          .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
          .allowed_header(header::CONTENT_TYPE)
          .max_age(3600)
      )
      // set up DB pool to be used with web::Data<Pool> extractor
      .data(pool.clone())
      .wrap(middleware::Logger::default())
      .configure(to_do::routes::config)
      .service(get_related_by_id)
      .service(post_references)
      .service(create_user)
      .service(login_user)
    // .service(delete_item)
  })
  .bind(&addr)?
  .run()
  .await
}
