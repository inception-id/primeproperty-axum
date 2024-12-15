// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        email -> Varchar,
    }
}
