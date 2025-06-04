diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        genre -> Varchar,
        published -> Bool,
    }
}