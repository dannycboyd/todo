-- Your SQL goes here
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    start date NOT NULL,
    repeats character(1) DEFAULT 'n' NOT NULL,
    title character varying(80) NOT NULL,
    note character varying(1000) NOT NULL,
    finished boolean DEFAULT false NOT NULL,
    CONSTRAINT check_repetition CHECK (((repeats)::text = ANY ((ARRAY['y'::character varying, 'm'::character varying, 'w'::character varying, 'd'::character varying, 'n'::character varying, 'e'::character varying])::text[])))
);

CREATE TABLE task_completions (
  id SERIAL PRIMARY KEY,
  task_id integer NOT NULL,
  date date NOT NULL,
  CONSTRAINT task_completions_unique UNIQUE (task_id, date),
  CONSTRAINT task_completions_task_id_fkey FOREIGN KEY (task_id) REFERENCES tasks(id)
);
