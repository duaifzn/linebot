use crate::database_pool::DatabasePool;
use crate::model::permission::{NewPermission, ReportPermission};
use crate::schema::permission::permissions;
use diesel::prelude::*;
use rocket::State;

pub fn insert_one_permission(
    db_pool: &State<DatabasePool>,
    new_group_id: &str,
) -> Result<bool, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            let find_result = permissions::dsl::permissions
                .select(permissions::group_id)
                .filter(permissions::group_id.eq(new_group_id))
                .load::<String>(&mut conn);

            if let Ok(res) = find_result {
                if res.len() > 0 {
                    let result = diesel::update(permissions::dsl::permissions)
                        .filter(permissions::group_id.eq(new_group_id))
                        .set((
                            permissions::has_daily.eq(true),
                            permissions::has_monthly.eq(true),
                            permissions::has_weekly.eq(true),
                            permissions::in_the_group.eq(true),
                        ))
                        .execute(&mut conn);
                    match result {
                        Ok(_) => return Ok(true),
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                }
            }
            let new_permission = NewPermission {
                group_id: new_group_id,
                in_the_group: true,
                has_daily: true,
                has_weekly: true,
                has_monthly: true,
            };
            let result = diesel::insert_into(permissions::table)
                .values(new_permission)
                .execute(&mut conn);
            match result {
                Ok(_) => Ok(true),
                Err(err) => Err(format!("{:?}", err)),
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn update_one_permission(
    db_pool: &State<DatabasePool>,
    target_group_id: &str,
    in_the_group: Option<bool>,
    has_daily: Option<bool>,
    has_weekly: Option<bool>,
    has_monthly: Option<bool>,
) -> Result<bool, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            if in_the_group.is_some()
                && has_daily.is_some()
                && has_weekly.is_some()
                && has_monthly.is_some()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((
                        permissions::in_the_group.eq(in_the_group.unwrap()),
                        permissions::has_daily.eq(has_daily.unwrap()),
                        permissions::has_weekly.eq(has_weekly.unwrap()),
                        permissions::has_monthly.eq(has_monthly.unwrap()),
                    ))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_none()
                && has_daily.is_some()
                && has_weekly.is_some()
                && has_monthly.is_some()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((
                        permissions::has_daily.eq(has_daily.unwrap()),
                        permissions::has_weekly.eq(has_weekly.unwrap()),
                        permissions::has_monthly.eq(has_monthly.unwrap()),
                    ))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_some()
                && has_daily.is_none()
                && has_weekly.is_some()
                && has_monthly.is_some()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((
                        permissions::in_the_group.eq(in_the_group.unwrap()),
                        permissions::has_weekly.eq(has_weekly.unwrap()),
                        permissions::has_monthly.eq(has_monthly.unwrap()),
                    ))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_none()
                && has_daily.is_none()
                && has_weekly.is_some()
                && has_monthly.is_some()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((
                        permissions::has_weekly.eq(has_weekly.unwrap()),
                        permissions::has_monthly.eq(has_monthly.unwrap()),
                    ))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_some()
                && has_daily.is_some()
                && has_weekly.is_none()
                && has_monthly.is_some()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((
                        permissions::in_the_group.eq(in_the_group.unwrap()),
                        permissions::has_daily.eq(has_daily.unwrap()),
                        permissions::has_monthly.eq(has_monthly.unwrap()),
                    ))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_none()
                && has_daily.is_some()
                && has_weekly.is_none()
                && has_monthly.is_some()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((
                        permissions::has_daily.eq(has_daily.unwrap()),
                        permissions::has_monthly.eq(has_monthly.unwrap()),
                    ))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_some()
                && has_daily.is_none()
                && has_weekly.is_none()
                && has_monthly.is_some()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((
                        permissions::has_daily.eq(has_daily.unwrap()),
                        permissions::has_monthly.eq(has_monthly.unwrap()),
                    ))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_none()
                && has_daily.is_none()
                && has_weekly.is_none()
                && has_monthly.is_some()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((permissions::has_monthly.eq(has_monthly.unwrap()),))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_some()
                && has_daily.is_some()
                && has_weekly.is_some()
                && has_monthly.is_none()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((
                        permissions::in_the_group.eq(in_the_group.unwrap()),
                        permissions::has_daily.eq(has_daily.unwrap()),
                        permissions::has_weekly.eq(has_weekly.unwrap()),
                    ))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_none()
                && has_daily.is_some()
                && has_weekly.is_some()
                && has_monthly.is_none()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((
                        permissions::has_daily.eq(has_daily.unwrap()),
                        permissions::has_weekly.eq(has_weekly.unwrap()),
                    ))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_some()
                && has_daily.is_none()
                && has_weekly.is_some()
                && has_monthly.is_none()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((
                        permissions::in_the_group.eq(in_the_group.unwrap()),
                        permissions::has_weekly.eq(has_weekly.unwrap()),
                    ))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_none()
                && has_daily.is_none()
                && has_weekly.is_some()
                && has_monthly.is_none()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((permissions::has_weekly.eq(has_weekly.unwrap()),))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_some()
                && has_daily.is_some()
                && has_weekly.is_none()
                && has_monthly.is_none()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((
                        permissions::in_the_group.eq(in_the_group.unwrap()),
                        permissions::has_daily.eq(has_weekly.unwrap()),
                    ))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_none()
                && has_daily.is_some()
                && has_weekly.is_none()
                && has_monthly.is_none()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((permissions::has_daily.eq(has_weekly.unwrap()),))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if in_the_group.is_some()
                && has_daily.is_none()
                && has_weekly.is_none()
                && has_monthly.is_none()
            {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((permissions::in_the_group.eq(in_the_group.unwrap()),))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else {
                Ok(true)
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn find_all_report_status_and_in_group(
    db_pool: DatabasePool,
) -> Result<Vec<ReportPermission>, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            let result = permissions::dsl::permissions
                .filter(permissions::in_the_group.eq(true))
                .filter(
                    permissions::has_daily
                        .eq(true)
                        .or(permissions::has_weekly.eq(true))
                        .or(permissions::has_monthly.eq(true)),
                )
                .select(ReportPermission::as_select())
                .load::<ReportPermission>(&mut conn);
            match result {
                Ok(res) => Ok(res),
                Err(err) => Err(format!("{:?}", err)),
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn update_one_permission_of_has_daily_to_on(
    db_pool: &State<DatabasePool>,
    group_id: &str,
) -> Result<bool, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            let find_result = permissions::dsl::permissions
                .filter(permissions::group_id.eq(group_id))
                .select(permissions::group_id)
                .load::<String>(&mut conn);

            if let Ok(res) = find_result {
                if res.len() > 0 {
                    let result = diesel::update(permissions::dsl::permissions)
                        .filter(permissions::group_id.eq(group_id))
                        .set((permissions::has_daily.eq(true),))
                        .execute(&mut conn);
                    match result {
                        Ok(_) => return Ok(true),
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                } else {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn update_one_permission_of_has_daily_to_off(
    db_pool: &State<DatabasePool>,
    group_id: &str,
) -> Result<bool, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            let find_result = permissions::dsl::permissions
                .select(permissions::group_id)
                .filter(permissions::group_id.eq(group_id))
                .load::<String>(&mut conn);

            if let Ok(res) = find_result {
                if res.len() > 0 {
                    let result = diesel::update(permissions::dsl::permissions)
                        .filter(permissions::group_id.eq(group_id))
                        .set((permissions::has_daily.eq(false),))
                        .execute(&mut conn);
                    match result {
                        Ok(_) => return Ok(true),
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                } else {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn update_one_permission_of_has_weekly_to_on(
    db_pool: &State<DatabasePool>,
    group_id: &str,
) -> Result<bool, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            let find_result = permissions::dsl::permissions
                .select(permissions::group_id)
                .filter(permissions::group_id.eq(group_id))
                .load::<String>(&mut conn);

            if let Ok(res) = find_result {
                if res.len() > 0 {
                    let result = diesel::update(permissions::dsl::permissions)
                        .filter(permissions::group_id.eq(group_id))
                        .set((permissions::has_weekly.eq(true),))
                        .execute(&mut conn);
                    match result {
                        Ok(_) => return Ok(true),
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                } else {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn update_one_permission_of_has_weekly_to_off(
    db_pool: &State<DatabasePool>,
    group_id: &str,
) -> Result<bool, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            let find_result = permissions::dsl::permissions
                .select(permissions::group_id)
                .filter(permissions::group_id.eq(group_id))
                .load::<String>(&mut conn);

            if let Ok(res) = find_result {
                if res.len() > 0 {
                    let result = diesel::update(permissions::dsl::permissions)
                        .filter(permissions::group_id.eq(group_id))
                        .set((permissions::has_weekly.eq(false),))
                        .execute(&mut conn);
                    match result {
                        Ok(_) => return Ok(true),
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                } else {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn update_one_permission_of_has_monthly_to_on(
    db_pool: &State<DatabasePool>,
    group_id: &str,
) -> Result<bool, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            let find_result = permissions::dsl::permissions
                .select(permissions::group_id)
                .filter(permissions::group_id.eq(group_id))
                .load::<String>(&mut conn);

            if let Ok(res) = find_result {
                if res.len() > 0 {
                    let result = diesel::update(permissions::dsl::permissions)
                        .filter(permissions::group_id.eq(group_id))
                        .set((permissions::has_monthly.eq(true),))
                        .execute(&mut conn);
                    match result {
                        Ok(_) => return Ok(true),
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                } else {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn update_one_permission_of_has_monthly_to_off(
    db_pool: &State<DatabasePool>,
    group_id: &str,
) -> Result<bool, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            let find_result = permissions::dsl::permissions
                .select(permissions::group_id)
                .filter(permissions::group_id.eq(group_id))
                .load::<String>(&mut conn);

            if let Ok(res) = find_result {
                if res.len() > 0 {
                    let result = diesel::update(permissions::dsl::permissions)
                        .filter(permissions::group_id.eq(group_id))
                        .set((permissions::has_monthly.eq(false),))
                        .execute(&mut conn);
                    match result {
                        Ok(_) => return Ok(true),
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                } else {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}
