#[macro_use]
extern crate rocket;
mod controller;
mod dto;
mod util;
mod middleware;
const ACCESS_TOKEN: &str = "";
const SECRET: &str = "";

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            routes![
                controller::line_controller::webhook,
            ],
        )
        .manage(util::line_bot::LineBot::new(SECRET, ACCESS_TOKEN))
}
