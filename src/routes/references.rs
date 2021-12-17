use actix_web::{HttpResponse, web, Error};
use crate::models::reference::NewItemRef;
use crate::{DbPool, get_pool_connection, parse_json};
use crate::actions::item::{insert_reference, delete_references, delete_child_ref};

// #[post("/references/")]
pub async fn post_references(
  pool: web::Data<DbPool>,
  payload: web::Payload
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);

  let reference = parse_json::<NewItemRef>(payload).await?;

  let updated_item = web::block(move || insert_reference(reference, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;

  Ok(HttpResponse::Ok().json(updated_item))
}

// #[delete("/references")]
pub async fn delete_references_handler(
  pool: web::Data<DbPool>,
  payload: web::Payload
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);

  let refs = parse_json::<Vec<NewItemRef>>(payload).await?;

  let refs = web::block(move || delete_references(refs, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;
  Ok(HttpResponse::Ok().json(refs))
}

/**
 * Deletes a `reference` matching `child_id`
 */
pub async fn delete_child_refs_handler(
  pool: web::Data<DbPool>,
  child_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let child_id = child_id.into_inner();

  let response = web::block(move || delete_child_ref(child_id, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;
  Ok(HttpResponse::Ok().json(response))
}
