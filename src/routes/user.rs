use actix_web::{web, HttpResponse, Error};

use crate::{DbPool, get_pool_connection, parse_json};
use crate::models::user::{NewUserRequest, User, LoginRequest};
use crate::models::responses::Response;
use crate::actions::user::{create_user, login_user};

// #[post("/user/new")]
pub async fn create_user_handler(
  pool: web::Data<DbPool>,
  payload: web::Payload
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let mut request_body = parse_json::<NewUserRequest>(payload).await?;

  request_body.user.pwd_hash = None;
  request_body.user.pwd_salt = None;
  let new_user: User =
    web::block(move || create_user(&conn, request_body.user, request_body.password))
      .await
      .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
      })?;
  let res = Response {
    id: None,
    message: format!(
      "Successfully created new user {} {}",
      new_user.firstname, new_user.lastname
    ),
    value: None
  };
  Ok(HttpResponse::Ok().json(res))
}

// #[get("/user/login")]
pub async fn login_user_handler(
  pool: web::Data<DbPool>,
  query: web::Query<LoginRequest>
) -> Result<HttpResponse, Error> {
  let conn = get_pool_connection(pool);
  let user: LoginRequest = query.into_inner();

  let login = web::block(move || login_user(user.id, user.password, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;

  Ok(HttpResponse::Ok().body(format!("Login {}", login)))
}
