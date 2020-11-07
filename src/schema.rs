table! {
    item_references (id) {
        id -> Int4,
        created_at -> Timestamp,
        origin_id -> Int4,
        child_id -> Int4,
    }
}

table! {
    items (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        start_d -> Nullable<Timestamp>,
        end_d -> Nullable<Timestamp>,
        repeats -> Bpchar,
        title -> Varchar,
        note -> Nullable<Varchar>,
        marked_done -> Bool,
        deleted -> Bool,
        parent_id -> Nullable<Int4>,
        journal -> Bool,
        todo -> Bool,
        cal -> Bool,
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

allow_tables_to_appear_in_same_query!(item_references, items, task_completions, tasks,);
