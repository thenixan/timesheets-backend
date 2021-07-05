table! {
    tokens (token) {
        token -> Varchar,
        user_id -> Uuid,
        created_at -> Timestamp,
        last_used_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        secret -> Text,
    }
}

joinable!(tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    tokens,
    users,
);
