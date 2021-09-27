use actix_web::web;

pub mod items;
use crate::routes::items::*;

// route setup happens here
pub fn config(cfg: &mut web::ServiceConfig) {
  // println!("building routes");
  cfg.service(
    web::scope("") // empty route. This is so we can have /items, /user, etc.
      .service(
        web::scope("/items") // items top level route
          .service(
            web::resource("")
              .route(web::get().to(get_items_handler))
              .route(web::post().to(upsert_item_handler))
          )
          .service(
            // /items/{item_id}
            web::scope("/{item_id}").service(
              web::resource("")
                .route(web::get().to(get_item_by_id_handler))
                .route(web::delete().to(delete_item))
            )
          )
      )
      .service(
        web::scope("/testError").service(web::resource("").route(web::get().to(test_error)))
      ) // other routes will chain off of this with .service(web::scope("/..."))
        // .service(web::scope("/user"))
        // .service(web::scope("/references"))
  );
}
