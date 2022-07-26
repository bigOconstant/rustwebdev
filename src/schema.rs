table! {
    users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        created_on -> Timestamp,
        last_login -> Timestamp,
    }
}
