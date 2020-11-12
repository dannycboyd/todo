use argon2::{self, Config, ThreadMode, Variant, Version};

use crate::models::user::User;
use diesel::PgConnection;

fn get_random_buf() -> Result<[u8; 32], getrandom::Error> {
  let mut buf = [0u8; 32];
  getrandom::getrandom(&mut buf)?;
  Ok(buf)
}

pub fn create_user(
  conn: &PgConnection,
  user: User,
  password: String
) -> Result<(), diesel::result::Error> {
  // let password
  let salt = b"othersalt";
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
  let hash = argon2::hash_encoded(password, salt, &config).unwrap();
  let matches = argon2::verify_encoded(&hash, password).unwrap();
}
