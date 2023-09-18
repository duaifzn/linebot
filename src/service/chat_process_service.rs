use crate::database_pool::DatabasePool;
use crate::dto::chat_process_dto::ChatProcessServiceDto;
use crate::model::permission::{NewPermission, ReportPermission};
use crate::schema::permission::permissions;
use diesel::prelude::*;
use rocket::State;

pub fn update_one_permission_of_has_daily_to_on(
    db_pool: &State<DatabasePool>,
    group_id: &str,
) -> Result<ChatProcessServiceDto, String> {
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
                        Ok(_) => {
                            return Ok(ChatProcessServiceDto {
                                success: true,
                                msg: Some("daily report switch on".to_string()),
                            })
                        }
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                } else {
                    return Ok(ChatProcessServiceDto {
                        success: false,
                        msg: Some("group id not found".to_string()),
                    });
                }
            } else {
                return Ok(ChatProcessServiceDto {
                    success: false,
                    msg: Some("group id not found".to_string()),
                });
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn update_one_permission_of_has_daily_to_off(
    db_pool: &State<DatabasePool>,
    group_id: &str,
) -> Result<ChatProcessServiceDto, String> {
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
                        Ok(_) => {
                            return Ok(ChatProcessServiceDto {
                                success: true,
                                msg: Some("daily report switch off".to_string()),
                            })
                        }
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                } else {
                    return Ok(ChatProcessServiceDto {
                        success: false,
                        msg: Some("group id not found".to_string()),
                    });
                }
            } else {
                return Ok(ChatProcessServiceDto {
                    success: false,
                    msg: Some("group id not found".to_string()),
                });
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn update_one_permission_of_has_weekly_to_on(
    db_pool: &State<DatabasePool>,
    group_id: &str,
) -> Result<ChatProcessServiceDto, String> {
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
                        Ok(_) => {
                            return Ok(ChatProcessServiceDto {
                                success: true,
                                msg: Some("weekly report switch on".to_string()),
                            })
                        }
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                } else {
                    return Ok(ChatProcessServiceDto {
                        success: false,
                        msg: Some("group id not found".to_string()),
                    });
                }
            } else {
                return Ok(ChatProcessServiceDto {
                    success: false,
                    msg: Some("group id not found".to_string()),
                });
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn update_one_permission_of_has_weekly_to_off(
    db_pool: &State<DatabasePool>,
    group_id: &str,
) -> Result<ChatProcessServiceDto, String> {
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
                        Ok(_) => {
                            return Ok(ChatProcessServiceDto {
                                success: true,
                                msg: Some("weekly report switch off".to_string()),
                            })
                        }
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                } else {
                    return Ok(ChatProcessServiceDto {
                        success: false,
                        msg: Some("group id not found".to_string()),
                    });
                }
            } else {
                return Ok(ChatProcessServiceDto {
                        success: false,
                        msg: Some("group id not found".to_string()),
                    });
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn update_one_permission_of_has_monthly_to_on(
    db_pool: &State<DatabasePool>,
    group_id: &str,
) -> Result<ChatProcessServiceDto, String> {
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
                        Ok(_) => {
                            return Ok(ChatProcessServiceDto {
                                success: true,
                                msg: Some("monthly report switch on".to_string()),
                            })
                        }
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                } else {
                    return Ok(ChatProcessServiceDto {
                        success: false,
                        msg: Some("group id not found".to_string()),
                    });
                }
            } else {
                return Ok(ChatProcessServiceDto {
                        success: false,
                        msg: Some("group id not found".to_string()),
                    });
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn update_one_permission_of_has_monthly_to_off(
    db_pool: &State<DatabasePool>,
    group_id: &str,
) -> Result<ChatProcessServiceDto, String> {
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
                        Ok(_) => {
                            return Ok(ChatProcessServiceDto {
                                success: true,
                                msg: Some("monthly report switch off".to_string()),
                            })
                        }
                        Err(err) => return Err(format!("{:?}", err)),
                    }
                } else {
                    return Ok(ChatProcessServiceDto {
                        success: false,
                        msg: Some("group id not found".to_string()),
                    });
                }
            } else {
                return Ok(ChatProcessServiceDto {
                        success: false,
                        msg: Some("group id not found".to_string()),
                    });
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}
