use crate::database_pool::DatabasePool;
use crate::model::email::{Email, NewEmail};
use crate::schema;
use diesel::{self, RunQueryDsl, QueryResult};
use rocket::State;

pub fn insert_one_email(db_pool: &State<DatabasePool>, email: &str) ->QueryResult<usize>{
    let mut conn = db_pool.pool.get().unwrap();
    let new_email = NewEmail { email: email };
    let result = diesel::insert_into(schema::email::emails::table)
        .values(new_email)
        .execute(&mut conn);
    result
}

pub fn find_one_email(db_pool: &State<DatabasePool>, email: &str) ->QueryResult<usize>{
    let mut conn = db_pool.pool.get().unwrap();
    let new_email = NewEmail { email: email };
    let result = diesel::insert_into(schema::email::emails::table)
        .values(new_email)
        .execute(&mut conn);
    result
}