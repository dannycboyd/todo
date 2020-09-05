extern crate chrono;
extern crate to_do;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use diesel::r2d2::{self, ConnectionManager};

use dotenv::dotenv;

#[macro_use] extern crate diesel_migrations;
use diesel::PgConnection;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use to_do::actions;
use to_do::models::{task, item, reference};

embed_migrations!();

// break these out into modules (by path, maybe?)
#[get("/item/{item_id}")]
async fn get_item_by_id(
    pool: web::Data<DbPool>,
    path_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let item_id = path_id.into_inner(); // https://chrismcg.com/2019/04/30/deserializing-optional-datetimes-with-serde/ this link seems outdated, works ok without custom parse
    println!("get item by id {}", item_id);

    let item = web::block(move || actions::item::get_item_by_id(item_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    if let Some(item) = item {
        Ok(HttpResponse::Ok().json(item))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No item found with ID {}", item_id));
        Ok(res)
    }
}

#[derive(serde::Deserialize)]
pub struct AddItem {
    item: item::NewItemTz,
    refs: Vec<reference::NewItemRef>
}

#[post("/item")]
async fn add_item(
    pool: web::Data<DbPool>,
    form: web::Json<AddItem>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    
    let request_body = form.into_inner();
    let new_refs = request_body.refs;

    let item = item::NewItem::from(request_body.item);


    // use web::block to offload blocking Diesel code without blocking server thread
    let new_item = web::block(move || actions::item::upsert_item(item, new_refs, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(new_item))
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

    let addr = "0.0.0.0:8080"; // TODO: pull this from env
    println!("Listening on http://{}", addr);

    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(add_item)
            .service(get_item_by_id)
    })
    .bind(&addr)?
    .run()
    .await
}
