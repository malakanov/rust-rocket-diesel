// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        genre -> Text,
        published -> Bool,
    }
}
