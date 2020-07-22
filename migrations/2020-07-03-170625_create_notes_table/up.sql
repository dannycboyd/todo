-- Your SQL goes here
CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    body CHARACTER varying(5000) NOT NULL
);

CREATE TABLE refs (
    created_at TIMESTAMP NOT NULL,
    parent_task INTEGER,
    parent_note INTEGER,
    child_task INTEGER,
    child_note INTEGER,
    PRIMARY KEY (parent_task, parent_note, child_task, child_note),
    CONSTRAINT refs_parent_task_fkey FOREIGN KEY (parent_task) REFERENCES tasks(id),
    CONSTRAINT refs_parent_note_fkey FOREIGN KEY (parent_note) REFERENCES notes(id),
    CONSTRAINT refs_child_task_fkey FOREIGN KEY (child_task) REFERENCES tasks(id),
    CONSTRAINT refs_child_note_fkey FOREIGN KEY (child_note) REFERENCES notes(id),
    CONSTRAINT ref_xor_unique CHECK ( coalesce (parent_task, parent_note) is not null and parent_note*parent_task is null)
);
