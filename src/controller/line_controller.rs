use rocket::serde::json::Json;
use rocket::State;
use crate::dto::line_dto::WebhookDto;
use crate::dto::line_flex_message::{FlexMessage};
use crate::util::line_bot::{LineBot};
use crate::dto::linebot_process::LineBotProcess;

#[post("/webhook", format = "json", data="<body>")]
pub async fn webhook(line_bot: &State<LineBot>, body: Json<WebhookDto>){
    match body.events[0].message.text.clone(){
        Some(t) => {
            match t {
                i if i.contains(&LineBotProcess::Hello.to_string()) =>{
                    let a = line_bot.many_quetions_layout();
                    let _ = line_bot.reply_msg(&body.events[0].reply_token, a).await;
                },
                i if i.contains(&LineBotProcess::PartTimer.to_string()) =>{},
                i if i.contains(&LineBotProcess::OperationReport.to_string()) =>{},
                i if i.contains(&LineBotProcess::SalesCharts.to_string()) =>{},
                _ =>{}
            };
        },
        None => {
            println!("[info] webhook text is none.");
            return
        },
    }
    
}

#[get("/")]
pub fn index(line_bot: &State<LineBot>) ->Json<FlexMessage>{
    let a = line_bot.many_quetions_layout();
    Json(a)
}