use rocket::serde::{Deserialize, Serialize};
use crate::dto::line_action::Action;
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", tag = "type")]
pub enum FlexMessage {
    #[serde(rename = "flex")]
    Flex(Flex),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Flex {
    #[serde(rename = "altText")]
    pub alt_text: String,
    pub contents: Container,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", tag = "type")]
pub enum Container {
    #[serde(rename = "carousel")]
    Carousel(Carousel),
    #[serde(rename = "bubble")]
    Bubble(Bubble),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Carousel {
    pub contents: Vec<Container>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Bubble {
    pub hero: Component,
    pub body: Component,
    pub footer: Component,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", tag = "type")]
pub enum Component {
    #[serde(rename = "box")]
    Box(Box),
    #[serde(rename = "button")]
    Button(Button),
    #[serde(rename = "image")]
    Image(Image),
    #[serde(rename = "text")]
    Text(Text),
}

impl Component {
    pub fn new_box(layout: &str, contents: Vec<Component>) -> Self {
        Self::Box(Box {
            layout: layout.to_string(),
            contents,
        })
    }
    pub fn new_button(style: &str, color: &str, action: Action) -> Self {
        Self::Button(Button {
            style: style.to_string(),
            color: color.to_string(),
            action,
        })
    }
    pub fn new_image(url: &str, size: &str, aspect_ratio: &str, aspect_mode: &str) -> Self {
        Self::Image(Image {
            url: url.to_string(),
            size: size.to_string(),
            aspect_ratio: aspect_ratio.to_string(),
            aspect_mode: aspect_mode.to_string(),
        })
    }
    pub fn new_text(text: &str, wrap: Option<bool>) -> Self {
        Self::Text(Text {
            text: text.to_string(),
            wrap,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Text {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Box {
    pub layout: String,
    pub contents: Vec<Component>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Image {
    pub size: String,
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: String,
    #[serde(rename = "aspectMode")]
    pub aspect_mode: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Button {
    pub style: String,
    pub color: String,
    pub action: Action,
}