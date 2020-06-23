-- Your SQL goes here
CREATE TABLE public.tasks (
    id SERIAL PRIMARY KEY,
    start date NOT NULL,
    repeats character varying(7) DEFAULT 'n'::character varying,
    title character varying(80) NOT NULL,
    note character varying(1000),
    finished boolean DEFAULT false,
    CONSTRAINT check_repetition CHECK (((repeats)::text = ANY ((ARRAY['y'::character varying, 'm'::character varying, 'w'::character varying, 'd'::character varying, 'n'::character varying, 'e'::character varying])::text[])))
);

CREATE TABLE task_completions (
  id SERIAL PRIMARY KEY,
  task_id integer,
  date date
);

ALTER TABLE task_completions
  ADD CONSTRAINT task_completions_unique UNIQUE (task_id, date);

ALTER TABLE task_completions
  ADD CONSTRAINT task_completions_task_id_fkey FOREIGN KEY (task_id) REFERENCES tasks(id);