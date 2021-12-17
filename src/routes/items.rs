use actix_web::{HttpResponse, web, Error};
use crate::{DbPool, get_pool_connection};
use crate::actions::item::{
  delete_item_by_id, get_item_by_id, get_items, upsert_item, get_references_by_id
};
use crate::models::item::{ItemFilter, NewItem};
use crate::models::incoming_item::NewItemTz;
use crate::models::responses::{Response};

use log::{error};

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
