use crate::dto::{
    line_action::Action,
    line_dto::{EndpointInfoDto, PushDto, ReplyDto},
    line_flex_message::{Bubble, Carousel, Component, Container, Flex, FlexMessage},
};
use base64::encode;
use hmac::{Hmac, Mac};
use reqwest::{header, Client, Result};
use rocket::serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LineBot {
    pub secret: String,
    pub access_token: String,
}

impl LineBot {
    pub fn new(secret: &str, access_token: &str) -> Self {
        Self {
            secret: secret.to_string(),
            access_token: access_token.to_string(),
        }
    }
    pub fn signature_is_valid(&self, sign: String, body: String) -> bool {
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(body.as_bytes());

        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        let sign1 = encode(code_bytes);
        if sign1 == sign {
            return true;
        } else {
            return false;
        }
    }
    pub async fn webhook_endpoint_info(&self) -> Result<EndpointInfoDto> {
        let client = Client::new();
        let res = client
            .get("https://api.line.me/v2/bot/channel/webhook/endpoint")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await?
            .json::<EndpointInfoDto>()
            .await?;
        Ok(res)
    }
    pub async fn reply_msg<T: rocket::serde::Serialize>(&self, reply_token: Option<String>, msg: Vec<T>) {
        if reply_token.is_none(){
            return
        }
        let body = ReplyDto {
            reply_token: reply_token.unwrap().to_string(),
            messages: msg,
        };

        let client = Client::new();
        let res = client
            .post("https://api.line.me/v2/bot/message/reply")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await;
    }
    pub async fn push_msg<T: rocket::serde::Serialize>(&self, user_id: &str, msg: Vec<T>) {
        let body = PushDto {
            to: user_id.to_string(),
            messages: msg,
        };

        let client = Client::new();
        let _ = client
            .post("https://api.line.me/v2/bot/message/push")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await;
    }
    pub fn hello_layout(&self) ->FlexMessage{
        FlexMessage::Flex(Flex{
            alt_text: "hello layout.".to_string(),
            contents: Container::Carousel(Carousel {
                contents: vec![
                    Self::custom_bubble_and_btn_layout()
                ],
            }),
        })
    }
    pub fn period_of_operation_report_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "period of operation report layout.".to_string(),
            contents: Container::Carousel(Carousel {
                contents: vec![
                    Self::custom_bubble_layout(
                        "??????????????????",
                        vec![
                            "??????????????????????????????",
                            "?????????????????????????????????",
                            "???????????????????????????",
                            "????????????",
                            "??????????????????",
                            "?????????????????????Top10",
                            "??????????????????Top10",
                        ],
                    ),
                    Self::custom_bubble_layout(
                        "??????????????????",
                        vec![
                            "??????????????????????????????",
                            "??????????????????????????????",
                            "????????????????????????????????????",
                            "???????????????????????????????????????",
                            "???????????????-????????????",
                            "???????????????-????????????",
                            "???????????????-????????????",
                        ],
                    ),
                ],
            }),
        })
    }
    pub fn sum_of_operation_report_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "sum of operation report layout.".to_string(),
            contents: Container::Carousel(Carousel {
                contents: vec![
                    Self::custom_bubble_layout(
                        "??????????????????",
                        vec![
                            "??????????????????????????????",
                            "??????????????????",
                            "???????????????",
                            "?????????????????????",
                            "???????????????????????????",
                        ],
                    ),
                ],
            }),
        })
    }
    pub fn presentation_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "presentation layout.".to_string(),
            contents: Container::Carousel(Carousel {
                contents: vec![
                    Self::custom_bubble_layout(
                        "??????????????????",
                        vec![],
                    ),
                ],
            }),
        })
    }
    pub fn custom_bubble_layout(title: &str, texts: Vec<&str>) -> Container {
        Container::Bubble(Bubble {
            hero: None,
            header: Some(Component::new_box(
                "vertical",
                vec![Component::new_text(title, Some("center"), None)],
                None,
                None,
                None,
                Some("10px"),
            )),
            body: Some(Component::new_box(
                "vertical",
                vec![
                    Component::new_image(
                        "https://scdn.line-apps.com/n/channel_devcenter/img/fx/01_1_cafe.png",
                        "full",
                        "center",
                        "1:2",
                        "cover",
                    ),
                    Component::new_box(
                        "vertical",
                        Self::custom_many_btn_layout(texts),
                        Some("absolute"),
                        Some("100%"),
                        None,
                        None,
                    ),
                ],
                None,
                None,
                Some("425px"),
                Some("0px"),
            )),
            footer: None,
        })
    }
    pub fn custom_bubble_and_btn_layout() -> Container {
        Container::Bubble(Bubble {
            hero: None,
            header: Some(Component::new_box(
                "vertical",
                vec![Component::new_text("??????", Some("center"), None)],
                None,
                None,
                None,
                Some("10px"),
            )),
            body: Some(Component::new_box(
                "vertical",
                vec![
                    Component::new_image(
                        "https://scdn.line-apps.com/n/channel_devcenter/img/fx/01_1_cafe.png",
                        "full",
                        "center",
                        "1:2",
                        "cover",
                    ),
                    Component::new_box(
                        "vertical",
                        vec![
                            // Self::custom_one_msg_btn_layout("??????????????????"),
                            // Self::custom_one_msg_btn_layout("??????????????????"),
                            Self::custom_one_uri_btn_layout("???????????????", "https://pmotcloud.kyart.tw/"),
                        ],
                        Some("absolute"),
                        Some("100%"),
                        None,
                        None,
                    ),
                ],
                None,
                None,
                Some("425px"),
                Some("0px"),
            )),
            footer: None,
        })
    }
    pub fn custom_one_msg_btn_layout(text: &str) -> Component {
        Component::new_box(
            "vertical",
            vec![Component::new_button(
                "secondary",
                None,
                Some("sm"),
                Action::new_message(text, text),
            )],
            Some("relative"),
            Some("100%"),
            None,
            Some("10px"),
        )
    }
    pub fn custom_one_uri_btn_layout(text: &str, uri: &str) -> Component {
        Component::new_box(
            "vertical",
            vec![Component::new_button(
                "secondary",
                None,
                Some("sm"),
                Action::new_uri(text, uri),
            )],
            Some("relative"),
            Some("100%"),
            None,
            Some("10px"),
        )
    }
    pub fn custom_many_btn_layout(texts: Vec<&str>) -> Vec<Component> {
        let btns: Vec<Component> = texts
            .into_iter()
            .map(|t| Self::custom_one_msg_btn_layout(t))
            .collect();
        btns
    }
    pub fn maintain_text_layout() ->Component{
        Component::new_text("?????????", None, None)
    }
}