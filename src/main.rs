#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
mod controller;
mod dto;
mod util;
mod middleware;
mod config;
mod database_pool;
mod model;
mod schema;
mod service;
use crate::config::Config;
use dotenv::dotenv;

lazy_static!{
    static ref CONFIG: Config<'static>= Config::load();
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    //database::Database::connect(&CONFIG.mysql_url);
    rocket::build()
        .mount(
            "/api",
            routes![
                controller::line_controller::webhook,
            ],
        )
        .manage(util::line_bot::LineBot::new(&CONFIG.secret, &CONFIG.access_token))
        .manage(util::redis::Redis::connect(&CONFIG.redis_url).await)
        .manage(database_pool::DatabasePool::connect_mysql(&CONFIG.mysql_url))
}
