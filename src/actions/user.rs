use diesel::prelude::*;
use argon2::{self, Config, ThreadMode, Variant, Version};

use crate::models::user::{User, NewUser};
use diesel::PgConnection;

fn get_random_buf() -> Result<[u8; 32], getrandom::Error> {
  let mut buf = [0u8; 32];
  getrandom::getrandom(&mut buf)?;
  Ok(buf)
}

pub fn create_user(
  conn: &PgConnection,
  mut to_insert: NewUser,
  password: String
) -> Result<User, diesel::result::Error> {
  println!("Creating new user");
  let password = password.as_bytes();
  // let password
  let salt = get_random_buf().unwrap();
  let config = Config {
    variant: Variant::Argon2i,
    version: Version::Version13,
    mem_cost: 65536,
    time_cost: 10,
    lanes: 4,
    thread_mode: ThreadMode::Parallel,
    secret: &[],
    ad: &[],
    hash_length: 32
  };
  let hash = argon2::hash_encoded(password, &salt, &config).unwrap();
  println!("\n{}\n\n{:?}", hash, salt);
  let matches = argon2::verify_encoded(&hash, password).unwrap();
  println!("{}", matches);

  to_insert.pwd_hash = Some(hash);
  to_insert.pwd_salt = Some(salt.to_vec());

  use crate::schema::users::dsl::*;
  let new_user = diesel::insert_into(users)
    .values(to_insert)
    .get_result::<User>(conn)?;

  Ok(new_user)
}

pub fn login_user(
  login_id: i32,
  password: String,
  conn: &PgConnection
) -> Result<bool, diesel::result::Error> {
  let password = password.as_bytes();

  use crate::schema::users::dsl::*;
  let user_opt = users
    .filter(id.eq(login_id))
    .first::<User>(conn)
    .optional()?;

  // return the password match or false if not found
  Ok(if let Some(found_user) = user_opt {
    // this should generate a jwt session token from the user somehow, and save the token to a tokens table
    let matches = argon2::verify_encoded(&found_user.pwd_hash, password).unwrap();
    println!("{}", matches);
    matches
  } else {
    false
  })
}
