diesel::table!{
    permissions (id){
        id -> Int4,
        group_id -> Text,
        in_the_group -> Bool,
        has_permission -> Bool,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}