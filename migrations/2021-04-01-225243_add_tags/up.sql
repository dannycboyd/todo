CREATE TABLE tags
(
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  item_id INTEGER NOT NULL,
  tag CHARACTER varying(255) NOT NULL,
  CONSTRAINT tag_item_id FOREIGN KEY (item_id) REFERENCES items(id)
);
SELECT diesel_manage_updated_at('tags');

-- I need to be able to group items by tag
-- select * from items where id in (select id from tags where tag = $)
-- can use the same (silly) way of getting refs, use the selectedIds, select tags where item_id in selectedIds (there has to be a better way D:   )
-- could have a Tags table (with id) and an item_tags ref table