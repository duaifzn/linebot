diesel::table! {
    emails (id){
        id -> Int4,
        email -> Text,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}