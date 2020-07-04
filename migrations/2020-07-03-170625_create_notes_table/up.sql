-- Your SQL goes here
CREATE TABLE public.notes (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    body CHARACTER varying(5000)
)

CREATE TABLE public.references (
    created_at TIMESTAMP NOT NULL,
    parent_task INTEGER,
    parent_note INTEGER,
    child_task INTEGER,
    child_note INTEGER
    CONSTRAINT references_parent_task_fkey FOREIGN KEY (parent_task) REFERENCES tasks(id);
    CONSTRAINT references_parent_note_fkey FOREIGN KEY (parent_note) REFERENCES notes(id);
    CONSTRAINT references_child_task_fkey FOREIGN KEY (child_task) REFERENCES tasks(id);
    CONSTRAINT references_child_note_fkey FOREIGN KEY (child_note) REFERENCES notes(id);
)
