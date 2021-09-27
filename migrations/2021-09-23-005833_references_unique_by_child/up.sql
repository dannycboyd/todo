-- references table should be unique by child_id
ALTER TABLE item_references
  DROP CONSTRAINT item_references_pkey,
  ADD CONSTRAINT item_references_pkey PRIMARY KEY (child_id),
  DROP COLUMN id,
  ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;

SELECT diesel_manage_updated_at('item_references');

-- Make the titles longer so that they can function more as notes in their own right
ALTER TABLE items
    ALTER COLUMN title TYPE varchar(500);