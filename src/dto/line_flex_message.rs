use crate::dto::line_action::Action;
use rocket::serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde", tag = "type")]
pub enum FlexMessage {
    #[serde(rename = "flex")]
    Flex(Flex),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Flex {
    #[serde(rename = "altText")]
    pub alt_text: String,
    pub contents: Container,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde", tag = "type")]
pub enum Container {
    #[serde(rename = "carousel")]
    Carousel(Carousel),
    #[serde(rename = "bubble")]
    Bubble(Bubble),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Carousel {
    pub contents: Vec<Container>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Bubble {
    pub hero: Option<Component>,
    pub header: Option<Component>,
    pub body: Option<Component>,
    pub footer: Option<Component>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    #[serde(rename = "separator")]
    Separator(Separator),
}

impl Component {
    pub fn new_box(
        layout: &str,
        contents: Vec<Component>,
        position: Option<&str>,
        width: Option<&str>,
        height: Option<&str>,
        padding_all: Option<&str>,
    ) -> Self {
        Self::Box(Box {
            layout: layout.to_string(),
            contents,
            position: match position {
                Some(p) => Some(p.to_string()),
                None => None,
            },
            width: match width {
                Some(w) => Some(w.to_string()),
                None => None,
            },
            height: match height {
                Some(h) => Some(h.to_string()),
                None => None,
            },
            padding_all: match padding_all {
                Some(p) => Some(p.to_string()),
                None => None,
            },
        })
    }
    pub fn new_button(
        style: &str,
        color: Option<&str>,
        height: Option<&str>,
        action: Action,
    ) -> Self {
        Self::Button(Button {
            style: style.to_string(),
            color: match color {
                Some(c) => Some(c.to_string()),
                None => None,
            },
            height: match height {
                Some(h) => Some(h.to_string()),
                None => None,
            },
            action,
        })
    }
    pub fn new_image(
        url: &str,
        size: &str,
        align: &str,
        aspect_ratio: &str,
        aspect_mode: &str,
    ) -> Self {
        Self::Image(Image {
            url: url.to_string(),
            size: size.to_string(),
            align: align.to_string(),
            aspect_ratio: aspect_ratio.to_string(),
            aspect_mode: aspect_mode.to_string(),
        })
    }
    pub fn new_text(text: &str, align: Option<&str>, wrap: Option<bool>) -> Self {
        Self::Text(Text {
            text: text.to_string(),
            align: match align {
                Some(a) => Some(a.to_string()),
                None => None,
            },
            wrap,
        })
    }
    pub fn new_separator(margin: Option<&str>) -> Self {
        Self::Separator(Separator {
            margin: match margin {
                Some(a) => Some(a.to_string()),
                None => None,
            },
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Text {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Box {
    pub layout: String,
    pub contents: Vec<Component>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(rename = "paddingAll", skip_serializing_if = "Option::is_none")]
    pub padding_all: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Image {
    pub size: String,
    pub align: String,
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: String,
    #[serde(rename = "aspectMode")]
    pub aspect_mode: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Button {
    pub style: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Separator {
    pub margin: Option<String>,
}
