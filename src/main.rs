#[macro_use]
extern crate rocket;
mod controller;
mod dto;
mod util;
const access_token: &str = "";
const secret: &str = "";

#[launch]
async fn rocket() -> _ {
    
    rocket::build()
        .mount(
            "/api",
            routes![
                controller::line_controller::webhook,
                controller::line_controller::index,
            ],
        )
        .manage(util::line_bot::LineBot::new(secret, access_token))
}
