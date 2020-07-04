table! {
    tasks (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        title -> Varchar,
        is_done -> Bool,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
