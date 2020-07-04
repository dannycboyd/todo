// If you're getting weird build errors (casting between column types)
// it's because you have a crossed wire between this macro def and your model.rs Model struct. Reorder your columns and the issue will be fixed.
table! {
    task_completions (id) {
        id -> Int4,
        date -> Date,
        task_id -> Int4,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        start -> Date,
        repeats -> Varchar,
        title -> Varchar,
        note -> Varchar,
        finished -> Bool,
    }
}

table! {
    note (id) {
        id -> Int4,
        body -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp
    }
}

// table! {
//     references (id) {
//         id -> Int4,
//         parent_task -> int4,
//         parent_note -> int4,
//         child_task -> int4,
//         child_note -> int4
//     }
// }

joinable!(task_completions -> tasks (task_id));

allow_tables_to_appear_in_same_query!(
    task_completions,
    tasks,
);
