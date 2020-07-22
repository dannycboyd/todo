table! {
    notes (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        body -> Varchar,
    }
}

table! {
    refs (parent_task, parent_note, child_task, child_note) {
        created_at -> Timestamp,
        parent_task -> Int4,
        parent_note -> Int4,
        child_task -> Int4,
        child_note -> Int4,
    }
}

table! {
    task_completions (id) {
        id -> Int4,
        task_id -> Int4,
        date -> Date,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        start -> Date,
        repeats -> Bpchar,
        title -> Varchar,
        note -> Varchar,
        finished -> Bool,
    }
}

joinable!(task_completions -> tasks (task_id));

allow_tables_to_appear_in_same_query!(
    notes,
    refs,
    task_completions,
    tasks,
);
