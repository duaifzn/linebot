use crate::database_pool::DatabasePool;
use crate::dto::line_dto::WebhookDto;
use crate::dto::line_message::{Message, Text};
use crate::dto::linebot_message::{LineBotMessage, PeriodOfOperationReport, SumOfOperationReport};
use crate::dto::linebot_process::LineBotProcess;
use crate::service::{email_service, permission_service};
use crate::util::line_bot::LineBot;
use crate::util::redis::Redis;
use rocket::State;

#[post("/webhook", data = "<body>")]
pub async fn webhook(
    line_bot: &State<LineBot>,
    db_pool: &State<DatabasePool>,
    redis: &State<Redis>,
    body: WebhookDto,
) {
    if body.events.len() == 0 {
        return;
    };
//     WebhookDto { destination: "U8734b4cb4e1f20b637c1cedfea79fd0e", events: [Event { event_type: "join", message: None, timestamp: 1690344484297, source: Source { source_type: "group", user_id: None, group_id: Some("Cec568323a26ed02971fec071e842be44") }, reply_token: Some("5ed49f066a704c2e82bc16e3f4691d5b"), mode: "active", webhook_event_id: "01H6863VMWQZBA0F5A29AGV5VH", delivery_context: DeliveryContext { is_redelivery: false } }] }
// thread 'rocket-worker-thread' panicked at 'called `Option::unwrap()` on a `None` value', src/controller/line_controller.rs:74:42
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// WebhookDto { destination: "U8734b4cb4e1f20b637c1cedfea79fd0e", events: [Event { event_type: "leave", message: None, timestamp: 1690344518627, source: Source { source_type: "group", user_id: None, group_id: Some("Cec568323a26ed02971fec071e842be44") }, reply_token: None, mode: "active", webhook_event_id: "01H6864XEJAHGVDB0ANYZS0S8A", delivery_context: DeliveryContext { is_redelivery: false } }] }
// thread 'rocket-worker-thread' panicked at 'called `Option::unwrap()` on a `None` value', src/controller/line_controller.rs:74:42
    
    // let check_userid = redis
    //     .sismember(
    //         &LineBotProcess::ValidUserId.to_string(),
    //         &body.events[0].source.user_id.as_ref().unwrap(),
    //     )
    //     .await;
    // match check_userid {
    //     Ok(result) => match result {
    //         true => {}
    //         false => {
    //             let exist = email_service::find_one_email(
    //                 db_pool,
    //                 &body.events[0].message.clone().unwrap().text.unwrap(),
    //             )
    //             .unwrap();
    //             if exist == true {
    //                 let _ = redis
    //                     .sadd(
    //                         &LineBotProcess::ValidUserId.to_string(),
    //                         &body.events[0].source.user_id.clone().unwrap(),
    //                     )
    //                     .await;
    //                 let _ = line_bot
    //                     .reply_msg(
    //                         body.events[0].reply_token.clone(),
    //                         vec![
    //                             Message::Text(Text::new("資料驗證中...")),
    //                             Message::Text(Text::new("驗證成功歡迎使用，可使用表單選取功能")),
    //                         ],
    //                     )
    //                     .await;
    //                 return;
    //             } else {
    //                 let _ = line_bot
    //                     .reply_msg(
    //                         body.events[0].reply_token.clone(),
    //                         vec![
    //                             Message::Text(Text::new("驗證失敗")),
    //                             Message::Text(Text::new(
    //                                 "你好，歡迎使用數位打工人，請確認您的身份",
    //                             )),
    //                             Message::Text(Text::new("請輸入計畫管理平台的mail進行驗證")),
    //                         ],
    //                     )
    //                     .await;
    //                 return;
    //             }
    //         }
    //     },
    //     Err(_) => return,
    // }
    if body.events[0].event_type == "join" && body.events[0].source.source_type == "group"{
        match &body.events[0].source.group_id {
            Some(id) => {
                let _ = permission_service::insert_one_permission(db_pool, id);
            }
            None =>{} 
        }
        
    }
    if body.events[0].event_type == "leave" && body.events[0].source.source_type == "group"{
        match &body.events[0].source.group_id {
            Some(id) => {
                let _ = permission_service::update_one_permission(db_pool, id, Some(false), Some(false));
            }
            None =>{} 
        }
    }
    match body.events[0].message{
        Some(_) =>{},
        None => return
    }
    match body.events[0].message.clone().unwrap().text {
        Some(t) => {
            match t {
                // i if i.contains(&LineBotMessage::Hello.to_string()) => {
                //     let a = line_bot.hello_layout();
                //     let _ = line_bot
                //         .reply_msg(body.events[0].reply_token.clone(), vec![a])
                //         .await;
                // }
                // i if i == LineBotMessage::PartTimer.to_string() => {
                //     let a = line_bot.hello_layout();
                //     let _ = line_bot
                //         .reply_msg(body.events[0].reply_token.clone(), vec![a])
                //         .await;
                // }
                i if i == LineBotMessage::PeriodOfOperationReport.to_string() => {
                    let a = line_bot.period_of_operation_report_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == LineBotMessage::Presentation.to_string() => {
                    let a = line_bot.presentation_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == LineBotMessage::SumOfOperationReport.to_string() => {
                    let a = line_bot.sum_of_operation_report_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == LineBotMessage::Logout.to_string() => {
                    let logout = redis
                        .srem(
                            &LineBotProcess::ValidUserId.to_string(),
                            &body.events[0].source.user_id.as_ref().unwrap(),
                        )
                        .await;
                    match logout {
                        Ok(result) => match result {
                            true => {
                                let _ = line_bot
                                    .reply_msg(
                                        body.events[0].reply_token.clone(),
                                        vec![Message::Text(Text::new("登出成功"))],
                                    )
                                    .await;
                            }
                            false => {
                                let _ = line_bot
                                    .reply_msg(
                                        body.events[0].reply_token.clone(),
                                        vec![Message::Text(Text::new("登出失敗"))],
                                    )
                                    .await;
                            }
                        },
                        Err(_) => {
                            let _ = line_bot
                                .reply_msg(
                                    body.events[0].reply_token.clone(),
                                    vec![Message::Text(Text::new("登出失敗"))],
                                )
                                .await;
                        }
                    }
                }
                i if i
                    == PeriodOfOperationReport::DayOfClassOfCustomerServiceAnalysis.to_string() =>
                {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::DayOfCustomerServiceAnalysis.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::DayOfRewardPoint.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::DayOfTrafficAnalysis.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::OrderTrend.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::RevenueOfRewardPoint.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::RevokeRewardPointTable.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::SalesChartsTop10.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::SendRewardPointTable.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::StatusOfRewardPoint.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::TotalOfOrderAnalysis.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::TrackOfSendRewardPoint.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::UseRewardPointTable.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == PeriodOfOperationReport::WeekOfSalesChartsTop10.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == SumOfOperationReport::CustomerServiceStatic.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == SumOfOperationReport::SalesCharts.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == SumOfOperationReport::SubsidyFirm.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == SumOfOperationReport::SumOfRevenue.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == SumOfOperationReport::SumOfRewardPoint.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                _ => {}
            };
        }
        None => {
            println!("[info] webhook text is none.");
            return;
        }
    }
}
