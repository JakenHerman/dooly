diesel::table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        priority -> Nullable<Integer>,
        due_date -> Nullable<Date>,
        completed -> Bool,
        user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Text,
    }
}

diesel::joinable!(todos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
