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
// use item::NewItem;
use reference::NewItemRef;
// type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use to_do::{DbPool, get_pool_connection};

use to_do::actions;
use to_do::models::{item, reference, user, responses};

embed_migrations!();

/**
 * returns an object with two fields: `items`: Vec<Item>, and `references`: Vec<ItemRef>
 */
#[get("/items/get")] // this needs some better controls on it. Pagination? Time segments?
async fn get_items(
  filters: web::Query<item::ItemFilter>,
  pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
  let filters = filters.into_inner();
  let conn = get_pool_connection(pool);
  let items = web::block(move || actions::item::get_items(&conn, filters))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;
  Ok(HttpResponse::Ok().json(items))
}

// break these out into modules (by path, maybe?)
#[get("/item/{item_id}")]
async fn get_item_by_id(
  pool: web::Data<DbPool>,
  path_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let item_id = path_id.into_inner(); // https://chrismcg.com/2019/04/30/deserializing-optional-datetimes-with-serde/ this link seems outdated, works ok without custom parse
  println!("get item by id {}", item_id);

  let item = web::block(move || actions::item::get_item_by_id(item_id, &conn))
    .await
    .map_err(|e| {
      // does this error pattern do what I hope it does?
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;
  if let Some(item) = item {
    Ok(HttpResponse::Ok().json(item))
  } else {
    let res = HttpResponse::NotFound().body(format!("No item found with ID {}", item_id));
    Ok(res)
  }
}

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
  form: web::Json<Vec<NewItemRef>>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);

  let refs = form.into_inner();

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
  form: web::Json<Vec<NewItemRef>>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);

  let refs = form.into_inner();

  let refs = web::block(move || actions::item::delete_references(refs, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;
  Ok(HttpResponse::Ok().json(refs))
}

#[derive(serde::Deserialize)]
pub struct AddItem {
  item: item::NewItemTz,
  refs: Vec<reference::NewItemRef>,
  tags: Vec<String>
}

#[post("/item")]
async fn add_item(
  pool: web::Data<DbPool>,
  form: web::Json<AddItem>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);

  let request_body = form.into_inner();
  let new_refs = request_body.refs;
  let tags = request_body.tags;

  let item = item::NewItem::from(request_body.item);

  // use web::block to offload blocking Diesel code without blocking server thread
  let new_item = web::block(move || actions::item::upsert_item(item, new_refs, tags, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;

  Ok(HttpResponse::Ok().json(new_item))
}

#[delete("/item/{item_id}")]
async fn delete_item(
  pool: web::Data<DbPool>,
  path_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let item_id = path_id.into_inner();

  let _item = web::block(move || actions::item::delete_item_by_id(item_id, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;
  let res = responses::Response {
    id: None,
    message: format!("Successfully deleted item #{}", item_id),
    value: Some(item_id)
  };
  Ok(HttpResponse::Ok().json(res))
}

#[post("/user/new")]
async fn create_user(
  pool: web::Data<DbPool>,
  form: web::Json<user::NewUserRequest>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let mut request_body = form.into_inner();

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

  // let addr = "0.0.0.0:8080"; // TODO: pull this from env
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
      .service(get_items)
      .service(add_item)
      .service(get_item_by_id)
      .service(get_related_by_id)
      .service(post_references)
      .service(create_user)
      .service(login_user)
      .service(delete_item)
  })
  .bind(&addr)?
  .run()
  .await
}
