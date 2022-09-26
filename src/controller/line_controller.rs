use crate::dto::line_dto::WebhookDto;
use crate::dto::linebot_process::LineBotProcess;
use crate::util::line_bot::LineBot;
use rocket::State;

#[post("/webhook", data = "<body>")]
pub async fn webhook(
    line_bot: &State<LineBot>,
    body: WebhookDto,
) {
    match body.events[0].message.text.clone() {
        Some(t) => {
            match t {
                i if i.contains(&LineBotProcess::Hello.to_string()) => {
                    let a = line_bot.many_quetions_layout();
                    let _ = line_bot.reply_msg(&body.events[0].reply_token, a).await;
                }
                i if i.contains(&LineBotProcess::PartTimer.to_string()) => {}
                i if i.contains(&LineBotProcess::OperationReport.to_string()) => {}
                i if i.contains(&LineBotProcess::SalesCharts.to_string()) => {}
                _ => {}
            };
        }
        None => {
            println!("[info] webhook text is none.");
            return;
        }
    }
}