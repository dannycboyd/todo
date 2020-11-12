-- This file should undo anything in `up.sql`

ALTER TABLE items
DROP COLUMN user_id;

DROP TABLE users;