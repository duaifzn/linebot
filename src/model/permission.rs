use diesel::prelude::*;
use rocket::{serde::Deserialize, time::Time};
use crate::schema::permission::permissions;

#[derive(Selectable, Queryable, PartialEq, Debug)]
pub struct Permission{
    pub id: u32,
    pub group_id: String,
    pub in_the_group: bool,
    pub has_daily: bool,
    pub has_weekly: bool,
    pub has_monthly: bool,
    pub create_at: Time,
    pub update_at: Time,
}

#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = permissions)]
pub struct NewPermission<'a>{
    pub group_id: &'a str,
    pub in_the_group: bool,
    pub has_daily: bool,
    pub has_weekly: bool,
    pub has_monthly: bool,
}
#[derive(Selectable, Queryable, AsChangeset, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = permissions)]
pub struct ReportPermission{
    pub group_id: String,
    pub has_daily: bool,
    pub has_weekly: bool,
    pub has_monthly: bool,
}