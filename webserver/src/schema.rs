// @generated automatically by Diesel CLI.

diesel::table! {
    repo (uuid) {
        uuid -> Text,
        name -> Text,
        username -> Text,
        email -> Text,
        branch -> Text,
        method -> Integer,
        status -> Text,
    }
}
