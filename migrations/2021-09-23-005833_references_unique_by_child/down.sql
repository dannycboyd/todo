
ALTER TABLE items
  ALTER COLUMN title TYPE varchar(80);

-- This file should undo anything in `up.sql`
ALTER TABLE item_references
  ADD COLUMN id SERIAL;

ALTER TABLE item_references
  DROP CONSTRAINT item_references_pkey,
  ADD CONSTRAINT item_references_pkey PRIMARY KEY (id);
DROP TRIGGER set_updated_at ON item_references;