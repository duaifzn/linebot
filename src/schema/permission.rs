diesel::table!{
    permissions (id){
        id -> Int4,
        group_id -> Text,
        in_the_group -> Bool,
        has_daily -> Bool,
        has_weekly -> Bool,
        has_monthly -> Bool,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}