// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (account_id) {
        account_id -> Int4,
        email -> Varchar,
    }
}

diesel::table! {
    todos (todo_id) {
        account_id -> Nullable<Int4>,
        todo_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        title -> Varchar,
        status -> Varchar,
    }
}

diesel::joinable!(todos -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    todos,
);
