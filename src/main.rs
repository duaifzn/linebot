#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
mod config;
mod controller;
mod database_pool;
mod dto;
mod middleware;
mod model;
mod schema;
mod service;
mod util;
use crate::config::Config;
use crate::util::schedule::{download_financial_report, broadcast_financial_report_to_group};
use dotenv::dotenv;
use std::thread::spawn;

lazy_static! {
    static ref CONFIG: Config<'static> = Config::load();
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    spawn(|| download_financial_report());
    spawn(|| broadcast_financial_report_to_group());
    rocket::build()
        .mount(
            "/api",
            routes![
                controller::line_controller::webhook,
                controller::report_controller::get_daily_report,
                controller::report_controller::get_weekly_report,
                controller::report_controller::get_monthly_report,
            ],
        )
        .manage(util::line_bot::LineBot::new(
            &CONFIG.secret,
            &CONFIG.access_token,
        ))
        .manage(util::redis::Redis::connect(&CONFIG.redis_url).await)
        .manage(database_pool::DatabasePool::connect_mysql(
            &CONFIG.mysql_url,
        ))
}
