use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", tag = "type")]
pub enum Action {
    #[serde(rename = "uri")]
    Uri(Uri),
    #[serde(rename = "message")]
    Message(Message)
}

impl Action {
    pub fn new_uri(label: &str, uri: &str) -> Self {
        Self::Uri(Uri{
            label: label.to_string(),
            uri: uri.to_string(),
        })
    }
    pub fn new_message(label: &str, text: &str) ->Self{
        Self::Message(Message{
            label: label.to_string(),
            text: text.to_string(),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Uri {
    pub label: String,
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub label: String,
    pub text: String,
}