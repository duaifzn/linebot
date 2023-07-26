use crate::database_pool::DatabasePool;
use crate::model::permission::NewPermission;
use crate::schema::permission::permissions;
use rocket::State;
use diesel::prelude::*;

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
                if res.len() > 0{
                    let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(new_group_id))
                    .set((
                        permissions::has_permission.eq(true),
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
                has_permission: true,
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
    new_in_the_group: Option<bool>,
    new_has_permission: Option<bool>,
) -> Result<bool, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            if new_has_permission.is_some() && new_in_the_group.is_none() {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set(permissions::has_permission.eq(new_has_permission.unwrap()))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if new_has_permission.is_none() && new_in_the_group.is_some() {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set(permissions::in_the_group.eq(new_in_the_group.unwrap()))
                    .execute(&mut conn);
                match result {
                    Ok(_) => return Ok(true),
                    Err(err) => return Err(format!("{:?}", err)),
                }
            } else if new_has_permission.is_some() && new_in_the_group.is_some() {
                let result = diesel::update(permissions::dsl::permissions)
                    .filter(permissions::group_id.eq(target_group_id))
                    .set((
                        permissions::has_permission.eq(new_has_permission.unwrap()),
                        permissions::in_the_group.eq(new_in_the_group.unwrap()),
                    ))
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

pub fn find_all_has_permission_and_in_group(
    db_pool: DatabasePool,
) -> Result<Vec<String>, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            let result = permissions::dsl::permissions
                .select(permissions::group_id)
                .filter(permissions::in_the_group.eq(true))
                .filter(permissions::has_permission.eq(true))
                .load::<String>(&mut conn);
            match result {
                Ok(res) => Ok(res),
                Err(err) => Err(format!("{:?}", err)),
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}
