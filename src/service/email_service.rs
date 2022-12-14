use crate::database_pool::DatabasePool;
use crate::model::email::{Email, NewEmail};
use crate::schema::email::emails;
use diesel::prelude::*;
use rocket::State;

pub fn insert_one_email(db_pool: &State<DatabasePool>, email: &str) -> Result<bool, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            let new_email = NewEmail { email: email };
            let result = diesel::insert_into(emails::table)
                .values(new_email)
                .execute(&mut conn);
            match result {
                Ok(_) => Ok(true),
                Err(err) => Err(format!("{:?}", err)),
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn find_one_email(db_pool: &State<DatabasePool>, email: &str) -> Result<bool, String> {
    let temp = db_pool.pool.get();
    match temp {
        Ok(mut conn) => {
            let result = emails::dsl::emails
                .select(emails::email)
                .filter(emails::email.eq(email))
                .load::<String>(&mut conn);
            match result {
                Ok(emails) => {
                    if emails.len() > 0{
                        return Ok(true)
                    }
                    else{
                        return Ok(false)
                    }
                },
                Err(err) => Err(format!("{:?}", err)),
            }
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}
