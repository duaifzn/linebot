use diesel::prelude::*;
use rocket::serde::Deserialize;
use crate::schema::email::emails;
use std::time::SystemTime;

#[derive(Queryable, Debug)]
pub struct Email{
    pub id: u32,
    pub email: String,
    pub create_at: SystemTime,
    pub update_at: SystemTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = emails)]
pub struct NewEmail<'a>{
    pub email: &'a str,
}