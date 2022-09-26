use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Event{
    #[serde(rename(deserialize = "type"))]
    pub event_type: String,
    pub message: Message,
    pub timestamp: u128,
    pub source: Source,
    #[serde(rename(deserialize = "replyToken"))]
    pub reply_token: String,
    pub mode: String,
    #[serde(rename(deserialize = "webhookEventId"))]
    pub webhook_event_id: String,
    #[serde(rename(deserialize = "deliveryContext"))]
    pub delivery_context: DeliveryContext
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Message{
    #[serde(rename(deserialize = "type"))]
    pub message_type: String,
    pub id: String,
    pub text: Option<String>,
    #[serde(rename(deserialize = "contentProvider"))]
    pub content_provider: Option<Povider>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Povider{
    #[serde(rename(deserialize = "type"))]
    pub povider_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Source{
    #[serde(rename(deserialize = "type"))]
    pub source_type: String,
    #[serde(rename(deserialize = "userId"))]
    pub user_id: Option<String>,
    #[serde(rename(deserialize = "groupId"))]
    pub group_id: Option<String>,

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeliveryContext{
    #[serde(rename(deserialize = "isRedelivery"))]
    pub is_redelivery: bool,
}