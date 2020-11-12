CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  start_d TIMESTAMP,
  end_d TIMESTAMP,
  repeats CHARACTER(1) DEFAULT 'n' NOT NULL,
  title CHARACTER varying(80) NOT NULL,
  note CHARACTER varying(1000),
  marked_done BOOLEAN DEFAULT false NOT NULL,
  deleted BOOLEAN DEFAULT false NOT NULL,
  parent_id INTEGER,
  -- flags for display type
  journal BOOLEAN DEFAULT false NOT NULL,
  todo BOOLEAN DEFAULT false NOT NULL,
  cal BOOLEAN DEFAULT false NOT NULL,

  CONSTRAINT item_repeats_check CHECK (((repeats)::text = ANY ((ARRAY['y'::character varying, 'm'::character varying, 'w'::character varying, 'd'::character varying, 'n'::character varying, 'e'::character varying])::text[]))),
  CONSTRAINT item_parent_fkey FOREIGN KEY (parent_id) REFERENCES items(id)
);
SELECT diesel_manage_updated_at('items');

CREATE TABLE item_references (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  origin_id INTEGER NOT NULL,
  child_id INTEGER NOT NULL,

  CONSTRAINT refs_child_item FOREIGN KEY (origin_id) REFERENCES items(id),
  CONSTRAINT refs_origin_item FOREIGN KEY (child_id) REFERENCES items(id)
);
