-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  firstname CHARACTER varying(25) NOT NULL,
  lastname CHARACTER varying(25) NOT NULL,
  prefix CHARACTER varying(10),
  note CHARACTER varying(1000),
  deleted BOOLEAN DEFAULT false NOT NULL,
  pwd_hash CHARACTER varying(50) NOT NULL,
  pwd_salt CHARACTER varying(50) NOT NULL

);
SELECT diesel_manage_updated_at('users');

ALTER TABLE items
  ADD user_id INTEGER REFERENCES users(id);
