use actix_web::web;

pub mod items;
pub mod references;
pub mod user;
use crate::routes::items::*;
use crate::routes::references::*;
use crate::routes::user::*;

// route setup happens here
pub fn config(cfg: &mut web::ServiceConfig) {
  // println!("building routes");

  /*
    call cfg.service and then you can chain:
      * scope ("/path").service().service()...
      * service ( scope("/more_path") )
      * service ( resource("/endpoint").route(...) )

    - use scope to set a node with several different subroutes
      - use service to group under that node
      ** DONT USE SCOPE FOR EMPTY ROUTE **
    - use resource("path").route()... .route() to set up a single endpoint with 1+ handlers

  */
  cfg.service(web::scope("/testError").service(web::resource("").route(web::get().to(test_error))));
  cfg.service(
    web::scope("/items") // items top level route
      .service(
        web::resource("")
          .route(web::get().to(get_items_handler))
          .route(web::post().to(upsert_item_handler))
      )
      .service(
        // /items/{item_id}
        web::scope("/{item_id}")
          .service(
            web::resource("")
              .route(web::get().to(get_item_by_id_handler))
              .route(web::delete().to(delete_item))
          )
          .service(web::resource("/related").route(web::get().to(get_related_by_id)))
      )
  );
  cfg.service(
    web::scope("/user")
      .service(web::resource("new").route(web::post().to(create_user_handler)))
      .service(web::resource("login").route(web::get().to(login_user_handler)))
  );
  cfg.service(
    web::scope("/references")
      .service(
        web::resource("").route(web::post().to(post_references)) // .route(web::delete().to(delete_references_handler))
      )
      .service(
        web::resource("child_id/{child_id}").route(web::delete().to(delete_child_refs_handler))
      )
  );
}
