table! {
    tasks (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        title -> Varchar,
        is_done -> Bool,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    users (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(tasks, users,);
