use crate::config::Config;
use crate::database_pool::DatabasePool;
use crate::service::permission_service;
use crate::util::chrome_bot::ChromeBot;
use crate::util::line_bot::LineBot;
use clokwerk::{AsyncScheduler, Job, TimeUnits};
use rocket::{tokio::time::sleep, tokio::time::Duration};

lazy_static! {
    static ref CONFIG: Config<'static> = Config::load();
}

pub async fn download_and_broadcast_report() {
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(1.day()).at("08:00:00").run(|| async {
        let bot = ChromeBot::new();
        match bot {
            Ok(b) => {
                let result = b.download_report();
                match result {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Error: download_financial_report {:?}", err)
                    }
                }
            }
            Err(err) => {
                println!("Error: chromeBot {:?}", err)
            }
        }
    });
    scheduler.every(1.day()).at("18:05:00").run(|| async {
        let bot = ChromeBot::new();
        match bot {
            Ok(b) => {
                let result = b.download_report();
                match result {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Error: download_financial_report {:?}", err)
                    }
                }
            }
            Err(err) => {
                println!("Error: chromeBot {:?}", err)
            }
        }
    });
    // scheduler.every(1.day()).at("09:00:00").run(|| async {
    //     let db_pool = DatabasePool::connect_mysql(&CONFIG.mysql_url);
    //     let line_bot = LineBot::new(&CONFIG.secret, &CONFIG.access_token);
    //     let result = permission_service::find_all_report_status_and_in_group(db_pool);
    //     match result {
    //         Ok(res) => {
    //             for r in res {
    //                 let layout =
    //                     LineBot::financial_report_layout(r.has_daily, r.has_weekly, r.has_monthly);
    //                 line_bot.push_msg(r.group_id.as_str(), vec![layout]).await;
    //                 println!("send report layout to group id {:?}", r.group_id);
    //                 sleep(Duration::from_millis(1000)).await;
    //             }
    //         }
    //         Err(err) => println!("Error: broadcast_financial_report_to_group {:?}", err),
    //     }
    // });
    loop {
        scheduler.run_pending().await;
        sleep(Duration::from_millis(1000)).await;
    }
}
