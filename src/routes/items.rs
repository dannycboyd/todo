use actix_web::{HttpResponse, web, Error};
use crate::{DbPool, get_pool_connection};
use crate::actions::item::{
  delete_item_by_id, get_item_by_id, get_items, upsert_item, get_references_by_id, shift_up,
  shift_down
};
use crate::models::item::{ItemFilter, NewItem};
use crate::models::incoming_item::NewItemTz;
use crate::models::responses::{Response};

use log::{debug, error};

pub async fn get_items_handler(
  pool: web::Data<DbPool>,
  filters: web::Query<ItemFilter>
) -> Result<HttpResponse, Error> {
  let filters = filters.into_inner();
  let conn = get_pool_connection(pool);
  let items = web::block(move || get_items(&conn, filters))
    .await
    .map_err(|e| {
      error!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;
  Ok(HttpResponse::Ok().json(items))
}

// #[post("/item")]
pub async fn upsert_item_handler(
  pool: web::Data<DbPool>,
  body: web::Payload
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);

  let request_body = crate::parse_json::<NewItemTz>(body).await?;

  let new_refs = request_body.refs.clone();
  let tags = request_body.tags.clone();

  // println!("gets here?");
  let item = NewItem::from(request_body);
  // gets bubbled straight back to the http response
  let result = web::block(move || upsert_item(item, new_refs, tags, &conn)).await;

  match result {
    Ok(item) => Ok(HttpResponse::Ok().json(item)),
    Err(e) => {
      error!("An error occurred: {}", e);
      Ok(HttpResponse::InternalServerError().finish())
    }
  }
}

pub async fn delete_item(
  pool: web::Data<DbPool>,
  path_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let item_id = path_id.into_inner();

  let _item = web::block(move || delete_item_by_id(item_id, &conn))
    .await
    .map_err(|e| {
      error!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;
  let res = Response {
    id: None,
    message: format!("Successfully deleted item #{}", item_id),
    value: Some(item_id)
  };
  Ok(HttpResponse::Ok().json(res))
}

pub async fn get_item_by_id_handler(
  pool: web::Data<DbPool>,
  path_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let item_id = path_id.into_inner();
  println!("get item by id {}", item_id);

  let item = web::block(move || get_item_by_id(item_id, &conn))
    .await
    .map_err(|e| {
      // does this error pattern do what I hope it does?
      error!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;
  if let Some(item) = item {
    Ok(HttpResponse::Ok().json(item))
  } else {
    let res = HttpResponse::NotFound().body(format!("No item found with ID {}", item_id));
    Ok(res)
  }
}

// #[get("/item/related/{item_id}")] // This is doing what the get item by id should be doing.
pub async fn get_related_by_id(
  pool: web::Data<DbPool>,
  path_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let item_id = path_id.into_inner();

  let item = web::block(move || get_references_by_id(item_id, &conn))
    .await
    .map_err(|e| {
      error!("{}", e);
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

#[derive(Debug, serde::Deserialize)]
pub struct ShiftRequest {
  dir: String
}

/**
 * this struct is to be used for returning the results of a shift request
 * item_one is the initial item
 * target is the item which has had its child_order updated as a side effect
 */
#[derive(serde::Serialize)]
struct ShiftResponse {
  item_one: i32,
  target: i32
}

impl From<(i32, i32)> for ShiftResponse {
  fn from(result: (i32, i32)) -> ShiftResponse {
    ShiftResponse {
      item_one: result.0,
      target: result.1
    }
  }
}

pub async fn shift_item_handler(
  pool: web::Data<DbPool>,
  item_id: web::Path<i32>,
  query: web::Query<ShiftRequest>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let item_id = item_id.into_inner();

  Ok(match query.dir.to_lowercase().as_str() {
    "up" => web::block(move || shift_up(item_id, &conn))
      .await
      .map(|result| HttpResponse::Ok().json(ShiftResponse::from(result)).into()),

    "down" => web::block(move || shift_down(item_id, &conn))
      .await
      .map(|result| HttpResponse::Ok().json(ShiftResponse::from(result)).into()),
    _ => Ok(HttpResponse::MethodNotAllowed().into())
  }?)
}

pub async fn indent_item_handler(
  pool: web::Data<DbPool>,
  item_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
  let _conn = get_pool_connection(pool);
  let item_id = item_id.into_inner();

  debug!("indent request - item id: {}", &item_id);

  Ok(HttpResponse::Ok().into())
}

pub async fn outdent_item_handler(
  pool: web::Data<DbPool>,
  item_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
  let _conn = get_pool_connection(pool);

  let item_id = item_id.into_inner();
  debug!("outdent request - item id: {}", &item_id);

  Ok(HttpResponse::Ok().into())
}

#[derive(serde::Serialize)]
struct JsonResponse {
  code: i32,
  err: Option<String>,
  msg: Option<String>
}

pub async fn test_error(_pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  error!("error endpoint called!");
  Ok(HttpResponse::BadRequest().json(JsonResponse {
    err: Some(String::from("An error occurred")),
    code: 500,
    msg: None
  }))
}
