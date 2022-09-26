use rocket::serde::{Deserialize, Serialize};
use crate::dto::line_event::Event;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct WebhookDto{
    pub destination: String,
    pub events: Vec<Event> 
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EndpointInfoDto {
    pub endpoint: String,
    pub active: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReplyDto<T> {
    #[serde(rename(serialize = "replyToken", deserialize = "replyToken"))]
    pub reply_token: String,
    pub messages: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PushDto<T> {
    pub to: String,
    pub messages: Vec<T>,
}