#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
mod controller;
mod dto;
mod util;
mod middleware;
mod config;
use crate::config::Config;
use dotenv::dotenv;

lazy_static!{
    static ref CONFIG: Config<'static>= Config::load();
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount(
            "/api",
            routes![
                controller::line_controller::webhook,
            ],
        )
        .manage(util::line_bot::LineBot::new(&CONFIG.secret, &CONFIG.access_token))
}
