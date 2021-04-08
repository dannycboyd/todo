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
        user_id -> Nullable<Int4>,
    }
}

table! {
    tags (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        item_id -> Int4,
        tag -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        firstname -> Varchar,
        lastname -> Varchar,
        prefix -> Nullable<Varchar>,
        note -> Nullable<Varchar>,
        deleted -> Bool,
        pwd_hash -> Varchar,
        pwd_salt -> Bytea,
    }
}

joinable!(items -> users (user_id));
joinable!(tags -> items (item_id));

allow_tables_to_appear_in_same_query!(
    item_references,
    items,
    tags,
    users,
);
