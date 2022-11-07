use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", tag= "type")]
pub enum Message {
    #[serde(rename = "text")]
    Text(Text)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Text{
    pub text: String
}

impl Text{
    pub fn new(msg: &str) ->Self{
        Self{
            text: msg.to_string()
        }
    }
}