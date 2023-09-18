use crate::database_pool::DatabasePool;
use crate::dto::chat_process_dto::ChatProcessDto;
use crate::dto::line_dto::WebhookDto;
use crate::dto::line_message::{Message, Text};
use crate::dto::linebot_message::{LineBotMessage, PeriodOfOperationReport, SumOfOperationReport};
use crate::service::permission_service::find_report_status_with_group_id_and_in_the_group;
use crate::service::{email_service, permission_service};
use crate::util::chat_process::ChatProcess;
use crate::util::line_bot::LineBot;
use crate::util::redis::Redis;
use rocket::State;

#[post("/webhook", data = "<body>")]
pub async fn webhook(
    line_bot: &State<LineBot>,
    db_pool: &State<DatabasePool>,
    redis: &State<Redis>,
    chat_process: &State<ChatProcess<'_>>,
    body: WebhookDto,
) {
    if body.events.len() == 0 {
        return;
    };

    if body.events[0].event_type == "join" && body.events[0].source.source_type == "group" {
        match &body.events[0].source.group_id {
            Some(id) => {
                let _ = permission_service::insert_one_permission(db_pool, id);
            }
            None => {}
        }
    }
    if body.events[0].event_type == "leave" && body.events[0].source.source_type == "group" {
        match &body.events[0].source.group_id {
            Some(id) => {
                let _ = permission_service::update_one_permission(
                    db_pool,
                    id,
                    Some(false),
                    None,
                    None,
                    None,
                );
            }
            None => {}
        }
    }
    match body.events[0].message {
        Some(_) => match body.events[0].message.clone().unwrap().text {
            Some(t) => match t {
                i if i == LineBotMessage::Setting.to_string() => {
                    let a = line_bot.setting_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == LineBotMessage::Report.to_string() => {
                    let report_status = find_report_status_with_group_id_and_in_the_group(
                        db_pool,
                        body.events[0].source.group_id.clone().unwrap().as_str(),
                    );
                    if let Ok(status) = report_status {
                        let layout = LineBot::financial_report_layout(
                            status.has_daily,
                            status.has_weekly,
                            status.has_monthly,
                        );
                        let _ = line_bot
                            .reply_msg(body.events[0].reply_token.clone(), vec![layout])
                            .await;
                    }
                }
                _ => {}
            },
            None => {
                println!("[info] webhook text is none.");
                return;
            }
        },
        None => {}
    }
    match body.events[0].postback {
        Some(_) => match body.events[0].postback.clone().unwrap().data {
            Some(t) => match t {
                i if i == ChatProcessDto::ScheduleDelivery.to_string() => {
                    let a = line_bot.schedule_delivery_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == ChatProcessDto::DailyReport.to_string() => {
                    let a = line_bot.daily_report_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == ChatProcessDto::WeeklyReport.to_string() => {
                    let a = line_bot.weekly_report_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == ChatProcessDto::MonthlyReport.to_string() => {
                    let a = line_bot.monthly_report_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), vec![a])
                        .await;
                }
                i if i == ChatProcessDto::DailySwitchOn.to_string()
                    || i == ChatProcessDto::DailySwitchOff.to_string()
                    || i == ChatProcessDto::WeeklySwitchOn.to_string()
                    || i == ChatProcessDto::WeeklySwitchOff.to_string()
                    || i == ChatProcessDto::MonthlySwitchOn.to_string()
                    || i == ChatProcessDto::MonthlySwitchOff.to_string() =>
                {
                    let result = chat_process.excute(i.as_str());
                    match result {
                        Some(f) => {
                            let chat_result = f(
                                db_pool,
                                body.events[0].source.group_id.clone().unwrap().as_str(),
                            );
                            match chat_result {
                                Ok(r) => match r.msg {
                                    Some(m) => {
                                        let _ = line_bot
                                            .reply_msg(
                                                body.events[0].reply_token.clone(),
                                                vec![Message::Text(Text::new(
                                                    m.to_string().as_str(),
                                                ))],
                                            )
                                            .await;
                                    }
                                    None => {}
                                },
                                Err(err) => {
                                    let _ = line_bot
                                        .reply_msg(
                                            body.events[0].reply_token.clone(),
                                            vec![Message::Text(Text::new(err.as_str()))],
                                        )
                                        .await;
                                }
                            }
                        }
                        None => {}
                    }
                }
                _ => {}
            },
            None => {
                println!("[info] webhook postback is none.");
                return;
            }
        },
        None => return,
    }
}

// match t {
//     // i if i.contains(&LineBotMessage::Hello.to_string()) => {
//     //     let a = line_bot.hello_layout();
//     //     let _ = line_bot
//     //         .reply_msg(body.events[0].reply_token.clone(), vec![a])
//     //         .await;
//     // }
//     // i if i == LineBotMessage::PartTimer.to_string() => {
//     //     let a = line_bot.hello_layout();
//     //     let _ = line_bot
//     //         .reply_msg(body.events[0].reply_token.clone(), vec![a])
//     //         .await;
//     // }
//     i if i == LineBotMessage::PeriodOfOperationReport.to_string() => {
//         let a = line_bot.period_of_operation_report_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == LineBotMessage::Presentation.to_string() => {
//         let a = line_bot.presentation_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == LineBotMessage::SumOfOperationReport.to_string() => {
//         let a = line_bot.sum_of_operation_report_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == LineBotMessage::Logout.to_string() => {
//         let logout = redis
//             .srem(
//                 &LineBotProcess::ValidUserId.to_string(),
//                 &body.events[0].source.user_id.as_ref().unwrap(),
//             )
//             .await;
//         match logout {
//             Ok(result) => match result {
//                 true => {
//                     let _ = line_bot
//                         .reply_msg(
//                             body.events[0].reply_token.clone(),
//                             vec![Message::Text(Text::new("登出成功"))],
//                         )
//                         .await;
//                 }
//                 false => {
//                     let _ = line_bot
//                         .reply_msg(
//                             body.events[0].reply_token.clone(),
//                             vec![Message::Text(Text::new("登出失敗"))],
//                         )
//                         .await;
//                 }
//             },
//             Err(_) => {
//                 let _ = line_bot
//                     .reply_msg(
//                         body.events[0].reply_token.clone(),
//                         vec![Message::Text(Text::new("登出失敗"))],
//                     )
//                     .await;
//             }
//         }
//     }
//     i if i
//         == PeriodOfOperationReport::DayOfClassOfCustomerServiceAnalysis.to_string() =>
//     {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::DayOfCustomerServiceAnalysis.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::DayOfRewardPoint.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::DayOfTrafficAnalysis.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::OrderTrend.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::RevenueOfRewardPoint.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::RevokeRewardPointTable.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::SalesChartsTop10.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::SendRewardPointTable.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::StatusOfRewardPoint.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::TotalOfOrderAnalysis.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::TrackOfSendRewardPoint.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::UseRewardPointTable.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == PeriodOfOperationReport::WeekOfSalesChartsTop10.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == SumOfOperationReport::CustomerServiceStatic.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == SumOfOperationReport::SalesCharts.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == SumOfOperationReport::SubsidyFirm.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == SumOfOperationReport::SumOfRevenue.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     i if i == SumOfOperationReport::SumOfRewardPoint.to_string() => {
//         let a = LineBot::maintain_text_layout();
//         let _ = line_bot
//             .reply_msg(body.events[0].reply_token.clone(), vec![a])
//             .await;
//     }
//     _ => {}
// };
