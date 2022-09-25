use crate::dto::{
    line_dto::{EndpointInfoDto, PushDto, ReplyDto},
    line_flex_message::{Bubble, Carousel, Component, Container, Flex, FlexMessage},
    line_action::Action,
};
use reqwest::{header, Client, Result};
use rocket::serde::{Deserialize, Serialize};

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
    pub async fn reply_msg(&self, reply_token: &str, msg: FlexMessage) {
        let body = ReplyDto {
            reply_token: reply_token.to_string(),
            messages: vec![msg]
        };

        let client = Client::new();
        let _ = client
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
    pub async fn push_msg(&self, user_id: &str, msg: FlexMessage) {
        let body: PushDto<FlexMessage> = PushDto {
            to: user_id.to_string(),
            messages: vec![msg],
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
    pub fn many_quetions_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "many quetions layout.".to_string(),
            contents: Container::Carousel(Carousel {
                contents: vec![Container::Bubble(Bubble{
                    hero: Component::new_image(
                        "https://scdn.line-apps.com/n/channel_devcenter/img/fx/01_5_carousel.png",
                        "full",
                        "20:13",
                        "cover",
                    ),
                    body: Component::new_box(
                        "vertical",
                        vec![
                            Component::new_text("Arm Chair, White", Some(true)),
                            Component::new_box(
                                "baseline",
                                vec![
                                    Component::new_text("$49", Some(true)),
                                ],
                            ),
                        ],
                    ),
                    footer: Component::new_box("vertical", vec![Component::new_button("primary", "#0000ff", Action::new_message("安安", "安安"))]),
            }),
            Container::Bubble(Bubble{
                hero: Component::new_image(
                    "https://scdn.line-apps.com/n/channel_devcenter/img/fx/01_6_carousel.png",
                    "full",
                    "20:13",
                    "cover",
                ),
                body: Component::new_box(
                    "vertical",
                    vec![
                        Component::new_text("Arm Chair, White", Some(true)),
                        Component::new_box(
                            "baseline",
                            vec![
                                Component::new_text("$49", Some(true)),
                            ],
                        ),
                    ],
                ),
                footer: Component::new_box("vertical", vec![Component::new_button("primary", "#0000ff", Action::new_uri("Add to wishlist", "https://linecorp.com"))]),
        })],
            }),
        })
    }
}
