use crate::database_pool::DatabasePool;
use crate::dto::line_dto::WebhookDto;
use crate::dto::linebot_process::{LineBotProcess, PeriodOfOperationReport, SumOfOperationReport};
use crate::dto::redis_dto::RedisSetName;
use crate::model::email::{Email, NewEmail};
use crate::util::line_bot::LineBot;
use crate::util::redis::Redis;
use crate::service::email_service;
use rocket::State;

#[post("/webhook", data = "<body>")]
pub async fn webhook(
    line_bot: &State<LineBot>,
    db_pool: &State<DatabasePool>,
    redis: &State<Redis>,
    body: WebhookDto,
) {
    if body.events[0].message.is_none() {
        return;
    };

    let check_userid = redis
        .sismember(
            &RedisSetName::ValidUserId.to_string(),
            &body.events[0].source.user_id.as_ref().unwrap(),
        )
        .await;
    match check_userid {
        Ok(_) => {}
        Err(_) => return,
    }

    match body.events[0].message.clone().unwrap().text {
        Some(t) => {
            match t {
                i if i.contains(&LineBotProcess::Hello.to_string()) => {
                    let a = line_bot.hello_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == LineBotProcess::PartTimer.to_string() => {
                    let a = line_bot.hello_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == LineBotProcess::PeriodOfOperationReport.to_string() => {
                    let a = line_bot.period_of_operation_report_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == LineBotProcess::Presentation.to_string() => {
                    let a = line_bot.presentation_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == LineBotProcess::SumOfOperationReport.to_string() => {
                    let a = line_bot.sum_of_operation_report_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i
                    == PeriodOfOperationReport::DayOfClassOfCustomerServiceAnalysis.to_string() =>
                {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::DayOfCustomerServiceAnalysis.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::DayOfRewardPoint.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::DayOfTrafficAnalysis.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::OrderTrend.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::RevenueOfRewardPoint.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::RevokeRewardPointTable.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::SalesChartsTop10.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::SendRewardPointTable.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::StatusOfRewardPoint.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::TotalOfOrderAnalysis.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::TrackOfSendRewardPoint.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::UseRewardPointTable.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == PeriodOfOperationReport::WeekOfSalesChartsTop10.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == SumOfOperationReport::CustomerServiceStatic.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == SumOfOperationReport::SalesCharts.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == SumOfOperationReport::SubsidyFirm.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == SumOfOperationReport::SumOfRevenue.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
                        .await;
                }
                i if i == SumOfOperationReport::SumOfRewardPoint.to_string() => {
                    let a = LineBot::maintain_text_layout();
                    let _ = line_bot
                        .reply_msg(body.events[0].reply_token.clone(), a)
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
