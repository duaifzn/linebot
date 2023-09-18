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
use crate::util::schedule::download_and_broadcast_report;
use dotenv::dotenv;
use rocket::tokio::spawn;
lazy_static! {
    static ref CONFIG: Config<'static> = Config::load();
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    spawn(async {
        download_and_broadcast_report().await;
    });
    rocket::build()
        .mount(
            "/api",
            routes![
                controller::line_controller::webhook,
                controller::report_controller::get_daily_report,
                controller::report_controller::get_weekly_report,
                controller::report_controller::get_monthly_report
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
        .manage(util::chat_process::ChatProcess::new())
}
