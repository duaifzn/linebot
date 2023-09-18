extern crate derive_more;
use crate::database_pool::DatabasePool;
use crate::dto::chat_process_dto::ChatProcessServiceDto;
use crate::service::chat_process_service::{
    update_one_permission_of_has_daily_to_off, update_one_permission_of_has_daily_to_on,
    update_one_permission_of_has_monthly_to_off, update_one_permission_of_has_monthly_to_on,
    update_one_permission_of_has_weekly_to_off, update_one_permission_of_has_weekly_to_on,
};
use derive_more::Display;
use rocket::State;
use std::sync::Arc;

#[derive(Display)]
pub enum ChatStep {
    #[display(fmt = "HEAD")]
    Head,
    #[display(fmt = "SETTING")]
    Setting,
    #[display(fmt = "VERIFY")]
    Verify,
    #[display(fmt = "SCHEDULEDELIVERY")]
    ScheduleDelivery,
    #[display(fmt = "DAILYREPORT")]
    DailyReport,
    #[display(fmt = "WEEKLYREPORT")]
    WeeklyReport,
    #[display(fmt = "MONTHLYREPORT")]
    MonthlyReport,
    #[display(fmt = "DAILYSWITCH")]
    DailySwitch,
    #[display(fmt = "WEEKLYSWITCH")]
    WeeklySwitch,
    #[display(fmt = "MONTHLYSWITCH")]
    MonthlySwitch,
    #[display(fmt = "ON")]
    On,
    #[display(fmt = "OFF")]
    Off,
}
//HEAD:SETTING:SCHEDULEDELIVERY:DAILYREPORT:DAILYSWITCH
// pub type BoxFuture<'a> = Pin<Box<dyn Future<Output = Result<bool, String>> + Send + Sync + 'a>>;
// pub type CustomFn<'a> = Arc<
//     dyn Fn(&'a State<LineBot>, &'a State<DatabasePool>, &'a str) -> BoxFuture<'a>
//         + Send
//         + Sync
//         + 'a,
// >;
pub struct ChatNode<'a> {
    step_name: ChatStep,
    next: Option<Arc<Vec<Arc<ChatNode<'a>>>>>,
    action: Option<
        Arc<dyn Fn(&State<DatabasePool>, &str) -> Result<ChatProcessServiceDto, String> + Send + Sync + 'a>,
    >,
}

pub struct ChatProcess<'a> {
    pub head: Arc<Vec<Arc<ChatNode<'a>>>>,
}

impl<'a> ChatProcess<'a> {
    pub fn new() -> Self {
        Self {
            head: Self::head_node(),
        }
    }
    pub fn excute(
        &self,
        msg: &str,
    ) -> Option<
        Arc<dyn Fn(&State<DatabasePool>, &str) -> Result<ChatProcessServiceDto, String> + Send + Sync + 'a>,
    > {
        let i: Vec<&str> = msg.split(":").collect();
        let mut head = self.head.clone();
        for node_name in i.into_iter() {
            let aa = head
                .iter()
                .find(|x| x.step_name.to_string().as_str() == node_name);
            match aa {
                Some(bb) => match &bb.next {
                    Some(cc) => {
                        head = cc.clone();
                    }
                    None => match &bb.action {
                        Some(ff) => return Some(ff.clone()),
                        None => return None,
                    },
                },
                None => return None,
            }
        }
        None
    }
    fn head_node() -> Arc<Vec<Arc<ChatNode<'a>>>> {
        Arc::new(vec![Arc::new(ChatNode {
            step_name: ChatStep::Head,
            next: Some(Arc::new(vec![
                Arc::new(Self::setting_node()),
                Arc::new(Self::verify_node())
            ])),
            action: None,
        })])
    }
    fn verify_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::Verify,
            next: None,
            action: None,
        }
    }
    fn setting_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::Setting,
            next: Some(Arc::new(vec![Arc::new(Self::schedule_delivery_node())])),
            action: None,
        }
    }
    fn schedule_delivery_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::ScheduleDelivery,
            next: Some(Arc::new(vec![
                Arc::new(Self::daily_report_node()),
                Arc::new(Self::weekly_report_node()),
                Arc::new(Self::monthly_report_node()),
            ])),
            action: None,
        }
    }
    fn daily_report_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::DailyReport,
            next: Some(Arc::new(vec![Arc::new(Self::daily_switch_node())])),
            action: None,
        }
    }
    fn weekly_report_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::WeeklyReport,
            next: Some(Arc::new(vec![Arc::new(Self::weekly_switch_node())])),
            action: None,
        }
    }
    fn monthly_report_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::MonthlyReport,
            next: Some(Arc::new(vec![Arc::new(Self::monthly_switch_node())])),
            action: None,
        }
    }
    fn daily_switch_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::DailySwitch,
            next: Some(Arc::new(vec![
                Arc::new(Self::daily_switch_on_node()),
                Arc::new(Self::daily_switch_off_node()),
            ])),
            action: None,
        }
    }
    fn weekly_switch_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::WeeklySwitch,
            next: Some(Arc::new(vec![
                Arc::new(Self::weekly_switch_on_node()),
                Arc::new(Self::weekly_switch_off_node()),
            ])),
            action: None,
        }
    }
    fn monthly_switch_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::MonthlySwitch,
            next: Some(Arc::new(vec![
                Arc::new(Self::monthly_switch_on_node()),
                Arc::new(Self::monthly_switch_off_node()),
            ])),
            action: None,
        }
    }
    fn daily_switch_on_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::On,
            next: None,
            action: Some(Arc::new(update_one_permission_of_has_daily_to_on)),
        }
    }
    fn daily_switch_off_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::Off,
            next: None,
            action: Some(Arc::new(update_one_permission_of_has_daily_to_off)),
        }
    }
    fn weekly_switch_on_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::On,
            next: None,
            action: Some(Arc::new(update_one_permission_of_has_weekly_to_on)),
        }
    }
    fn weekly_switch_off_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::Off,
            next: None,
            action: Some(Arc::new(update_one_permission_of_has_weekly_to_off)),
        }
    }
    fn monthly_switch_on_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::On,
            next: None,
            action: Some(Arc::new(update_one_permission_of_has_monthly_to_on)),
        }
    }
    fn monthly_switch_off_node() -> ChatNode<'a> {
        ChatNode {
            step_name: ChatStep::Off,
            next: None,
            action: Some(Arc::new(update_one_permission_of_has_monthly_to_off)),
        }
    }
}
