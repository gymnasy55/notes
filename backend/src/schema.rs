table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        encrypted_password -> Varchar,
        salt -> Varchar,
    }
}
