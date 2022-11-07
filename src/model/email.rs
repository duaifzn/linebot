use diesel::prelude::*;
use rocket::{serde::Deserialize, time::Time};
use crate::schema::email::emails;
use std::time::SystemTime;

#[derive(Queryable, PartialEq, Debug)]
pub struct Email{
    pub id: u32,
    pub email: String,
    pub create_at: Time,
    pub update_at: Time,
}

#[derive(Insertable, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = emails)]
pub struct NewEmail<'a>{
    pub email: &'a str,
}