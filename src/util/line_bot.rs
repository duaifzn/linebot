use crate::dto::chat_process_dto::ChatProcessDto;
use crate::dto::{
    line_action::Action,
    line_dto::{BroadcastDto, EndpointInfoDto, PushDto, ReplyDto},
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
    pub async fn reply_msg<T: rocket::serde::Serialize>(
        &self,
        reply_token: Option<String>,
        msg: Vec<T>,
    ) {
        if reply_token.is_none() {
            return;
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
        let a = client
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
    pub fn hello_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "hello layout.".to_string(),
            contents: Container::Carousel(Carousel {
                contents: vec![Self::custom_bubble_and_btn_layout()],
            }),
        })
    }
    pub fn period_of_operation_report_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "period of operation report layout.".to_string(),
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
                ],
            }),
        })
    }
    pub fn sum_of_operation_report_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "sum of operation report layout.".to_string(),
            contents: Container::Carousel(Carousel {
                contents: vec![Self::custom_bubble_layout(
                    "累計營運報表",
                    vec![
                        "累計點數營運數量統計",
                        "累計銷售情形",
                        "銷售排行榜",
                        "受補助企業分布",
                        "客服及諮詢服務狀況",
                    ],
                )],
            }),
        })
    }
    pub fn presentation_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "presentation layout.".to_string(),
            contents: Container::Carousel(Carousel {
                contents: vec![Self::custom_bubble_layout("累計營運報表", vec![])],
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
                vec![Component::new_text("報表", Some("center"), None)],
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
                            // Self::custom_one_msg_btn_layout("定期營運報表"),
                            // Self::custom_one_msg_btn_layout("累計營運報表"),
                            Self::custom_one_uri_btn_layout(
                                "雲市集營運報表-定期營運報表",
                                "https://dashboard.pmotcloud.org.tw/cloud/e1",
                            ),
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
    pub fn custom_one_postback_btn_layout(text: &str, data: &str) -> Component {
        Component::new_box(
            "vertical",
            vec![Component::new_button(
                "secondary",
                None,
                Some("sm"),
                Action::new_postback(text, data),
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
    pub fn maintain_text_layout() -> Component {
        Component::new_text("維護中", None, None)
    }
    pub fn financial_report_layout(
        daily_report: bool,
        weekly_report: bool,
        monthly_report: bool,
    ) -> FlexMessage {
        let mut btns: Vec<Component> = vec![];
        if daily_report {
            btns.append(&mut vec![Component::new_button(
                "link",
                None,
                Some("sm"),
                Action::new_uri("日報表", "https://xprooftest.com/api/report/daily"),
            )]);
        }
        if daily_report {
            btns.append(&mut vec![Component::new_button(
                "link",
                None,
                Some("sm"),
                Action::new_uri("週報表", "https://xprooftest.com/api/report/weekly"),
            )]);
        }
        if daily_report {
            btns.append(&mut vec![Component::new_button(
                "link",
                None,
                Some("sm"),
                Action::new_uri("月報表", "https://xprooftest.com/api/report/monthly"),
            )]);
        }
        FlexMessage::Flex(Flex {
            alt_text: "定期營運報表".to_string(),
            contents: Container::Bubble(Bubble {
                hero: Some(Component::new_image(
                    "https://i.imgur.com/xrfcRvV.jpg",
                    "full",
                    "center",
                    "3:1",
                    "cover",
                )),
                header: None,
                body: Some(Component::new_box(
                    "vertical",
                    vec![
                        Component::new_text("定期營運報表", Some("center"), None),
                        Component::new_separator(Some("sm")),
                        Component::new_box(
                            "vertical",
                            btns,
                            None,
                            None,
                            None,
                            None,
                        ),
                    ],
                    None,
                    None,
                    None,
                    None,
                )),
                footer: None,
            }),
        })
    }
    pub fn setting_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "設定".to_string(),
            contents: Container::Bubble(Bubble {
                hero: None,
                header: Some(Component::new_box(
                    "vertical",
                    vec![Component::new_text("設定", Some("center"), None)],
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
                                Self::custom_one_postback_btn_layout(
                                    "定時寄送",
                                    ChatProcessDto::ScheduleDelivery.to_string().as_str(),
                                ),
                                Self::custom_one_postback_btn_layout("呼喚寄送", "a"),
                                Self::custom_one_postback_btn_layout("其他", "b"),
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
            }),
        })
    }
    pub fn schedule_delivery_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "設定:定時寄送".to_string(),
            contents: Container::Bubble(Bubble {
                hero: None,
                header: Some(Component::new_box(
                    "vertical",
                    vec![Component::new_text("設定:定時寄送", Some("center"), None)],
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
                                Self::custom_one_postback_btn_layout(
                                    "日報",
                                    ChatProcessDto::DailyReport.to_string().as_str(),
                                ),
                                Self::custom_one_postback_btn_layout(
                                    "週報",
                                    ChatProcessDto::WeeklyReport.to_string().as_str(),
                                ),
                                Self::custom_one_postback_btn_layout(
                                    "月報",
                                    ChatProcessDto::MonthlyReport.to_string().as_str(),
                                ),
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
            }),
        })
    }
    pub fn daily_report_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "設定:定時寄送:日報".to_string(),
            contents: Container::Bubble(Bubble {
                hero: None,
                header: Some(Component::new_box(
                    "vertical",
                    vec![Component::new_text(
                        "設定:定時寄送:日報",
                        Some("center"),
                        None,
                    )],
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
                                Self::custom_one_postback_btn_layout(
                                    "開",
                                    ChatProcessDto::DailySwitchOn.to_string().as_str(),
                                ),
                                Self::custom_one_postback_btn_layout(
                                    "關",
                                    ChatProcessDto::DailySwitchOff.to_string().as_str(),
                                ),
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
            }),
        })
    }
    pub fn weekly_report_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "設定:定時寄送:週報".to_string(),
            contents: Container::Bubble(Bubble {
                hero: None,
                header: Some(Component::new_box(
                    "vertical",
                    vec![Component::new_text(
                        "設定:定時寄送:週報",
                        Some("center"),
                        None,
                    )],
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
                                Self::custom_one_postback_btn_layout(
                                    "開",
                                    ChatProcessDto::WeeklySwitchOn.to_string().as_str(),
                                ),
                                Self::custom_one_postback_btn_layout(
                                    "關",
                                    ChatProcessDto::WeeklySwitchOff.to_string().as_str(),
                                ),
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
            }),
        })
    }
    pub fn monthly_report_layout(&self) -> FlexMessage {
        FlexMessage::Flex(Flex {
            alt_text: "設定:定時寄送:月報".to_string(),
            contents: Container::Bubble(Bubble {
                hero: None,
                header: Some(Component::new_box(
                    "vertical",
                    vec![Component::new_text(
                        "設定:定時寄送:月報",
                        Some("center"),
                        None,
                    )],
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
                                Self::custom_one_postback_btn_layout(
                                    "開",
                                    ChatProcessDto::MonthlySwitchOn.to_string().as_str(),
                                ),
                                Self::custom_one_postback_btn_layout(
                                    "關",
                                    ChatProcessDto::MonthlySwitchOff.to_string().as_str(),
                                ),
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
            }),
        })
    }
}
