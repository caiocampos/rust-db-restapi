diesel::table! {
    clients (id) {
        id -> Integer,
        name -> Text,
        password -> Text,
        profession -> Nullable<Text>,
    }
}
