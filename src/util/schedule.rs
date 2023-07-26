use crate::config::Config;
use crate::util::chrome_bot::ChromeBot;
use crate::util::line_bot::LineBot;
use crate::service::permission_service;
use crate::database_pool::DatabasePool;
use clokwerk::{AsyncJob, AsyncScheduler, Job, Scheduler, TimeUnits};
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
            },
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

pub fn broadcast_financial_report_to_group() {
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(1.day()).at("07:00:00").run( || async {
        let db_pool = DatabasePool::connect_mysql(&CONFIG.mysql_url);
        let line_bot = LineBot::new(&CONFIG.secret, &CONFIG.access_token);
        let layout = LineBot::financial_report_layout();
        let result = permission_service::find_all_has_permission_and_in_group(db_pool);
        match result {
            Ok(group_ids) => {
                for group_id in group_ids{
                    line_bot.push_msg(&group_id, vec![layout.clone()]).await;
                }
            },
            Err(err) => println!("Error: broadcast_financial_report_to_group {:?}", err),
        }
    });
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(1000));
    }
}
