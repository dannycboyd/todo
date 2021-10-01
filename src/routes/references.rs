use actix_web::{HttpResponse, web, Error};
use crate::models::reference::NewItemRef;
use crate::{DbPool, get_pool_connection, parse_json};
use crate::actions::item::{insert_references, delete_references};

// #[post("/references/")]
pub async fn post_references(
  pool: web::Data<DbPool>,
  payload: web::Payload
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);

  let refs = parse_json::<Vec<NewItemRef>>(payload).await?;

  let refs = web::block(move || insert_references(refs, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;

  Ok(HttpResponse::Ok().json(refs))
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
