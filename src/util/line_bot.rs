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
    pub async fn reply_msg<T: rocket::serde::Serialize>(&self, reply_token: Option<String>, msg: T) {
        if reply_token.is_none(){
            return
        }
        let body = ReplyDto {
            reply_token: reply_token.unwrap().to_string(),
            messages: vec![msg],
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
    pub async fn push_msg<T: rocket::serde::Serialize>(&self, user_id: &str, msg: T) {
        let body = PushDto {
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
                contents: vec![
                    Self::custom_bubble_layout(
                        "定期營運報表",
                        vec![
                            "每日點數營運數量統計",
                            "點數發放進度追蹤與趨勢",
                            "點數發放與使用狀況",
                            "訂單趨勢",
                            "累計營運數字",
                            "累計銷售排行榜Top10",
                            "週銷售排行榜Top10",
                        ],
                    ),
                    Self::custom_bubble_layout(
                        "定期營運報表",
                        vec![
                            "每日網站流量數據分析",
                            "累計數位支付訂單分析",
                            "每日客服通話與服務量統計",
                            "每日客服來電類別服務量統計",
                            "點數管控表-發放情形",
                            "點數管控表-使用情形",
                            "點數管控表-核銷情形",
                        ],
                    ),
                    Self::custom_bubble_layout(
                        "累計營運報表",
                        vec![
                            "累計點數營運數量統計",
                            "累計銷售情形",
                            "銷售排行榜",
                            "受補助企業分布",
                            "客服及諮詢服務狀況",
                        ],
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
    pub fn custom_one_btn_layout(text: &str) -> Component {
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
    pub fn custom_many_btn_layout(texts: Vec<&str>) -> Vec<Component> {
        let btns: Vec<Component> = texts
            .into_iter()
            .map(|t| Self::custom_one_btn_layout(t))
            .collect();
        btns
    }
    pub fn maintain_text_layout() ->Component{
        Component::new_text("維護中", None, None)
    }
}