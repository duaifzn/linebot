use crate::config::Config;
use crate::database_pool::DatabasePool;
use crate::service::permission_service;
use crate::util::chrome_bot::ChromeBot;
use crate::util::line_bot::LineBot;
use clokwerk::{AsyncScheduler, Job, Scheduler, TimeUnits};
use std::{thread, time::Duration};

lazy_static! {
    static ref CONFIG: Config<'static> = Config::load();
}

pub fn download_financial_report() {
    let mut scheduler = Scheduler::new();
    scheduler.every(1.day()).at("06:00:00").run(|| {
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
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(1000));
    }
}

pub async fn broadcast_financial_report_to_group() {
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(1.day()).at("07:00:00").run(|| async {
        let db_pool = DatabasePool::connect_mysql(&CONFIG.mysql_url);
        let line_bot = LineBot::new(&CONFIG.secret, &CONFIG.access_token);
        let result = permission_service::find_all_report_status_and_in_group(db_pool);
        match result {
            Ok(res) => {
                for r in res {
                    println!("send report layout to group id {:?}", r.group_id);
                    let layout =
                        LineBot::financial_report_layout(r.has_daily, r.has_weekly, r.has_monthly);
                    line_bot.push_msg(&r.group_id, vec![layout]).await;
                }
            }
            Err(err) => println!("Error: broadcast_financial_report_to_group {:?}", err),
        }
    });
    loop {
        scheduler.run_pending().await;
        thread::sleep(Duration::from_millis(1000));
    }
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
    scheduler.every(1.day()).at("09:00:00").run(|| async {
        let db_pool = DatabasePool::connect_mysql(&CONFIG.mysql_url);
        let line_bot = LineBot::new(&CONFIG.secret, &CONFIG.access_token);
        let result = permission_service::find_all_report_status_and_in_group(db_pool);
        match result {
            Ok(res) => {
                for r in res {
                    println!("send report layout to group id {:?}", r.group_id);
                    let layout =
                        LineBot::financial_report_layout(r.has_daily, r.has_weekly, r.has_monthly);
                    line_bot.push_msg(&r.group_id, vec![layout]).await;
                }
            }
            Err(err) => println!("Error: broadcast_financial_report_to_group {:?}", err),
        }
    });
    loop {
        scheduler.run_pending().await;
        thread::sleep(Duration::from_millis(1000));
    }
}
