use derive_more::Display;

#[derive(Display)]
pub enum ChatProcessDto{
    #[display(fmt = "HEAD")]
    Head,
    #[display(fmt = "HEAD:SETTING")]
    Setting,
    #[display(fmt = "HEAD:VERIFY")]
    Verify,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY")]
    ScheduleDelivery,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY:DAILYREPORT")]
    DailyReport,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY:WEEKLTREPORT")]
    WeeklyReport,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY:MONTHLYREPORT")]
    MonthlyReport,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY:DAILYREPORT:DAILYSWITCH")]
    DailySwitch,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY:WEEKLTREPORT:WEEKLYSWITCH")]
    WeeklySwitch,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY:MONTHLYREPORT:MONTHLYSWITCH")]
    MonthlySwitch,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY:DAILYREPORT:DAILYSWITCH:ON")]
    DailySwitchOn,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY:DAILYREPORT:DAILYSWITCH:OFF")]
    DailySwitchOff,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY:WEEKLTREPORT:WEEKLYSWITCH:ON")]
    WeeklySwitchOn,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY:WEEKLTREPORT:WEEKLYSWITCH:OFF")]
    WeeklySwitchOff,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY:MONTHLYREPORT:MONYHLYSWITCH:ON")]
    MonthlySwitchOn,
    #[display(fmt = "HEAD:SETTING:SCHEDULEDELIVERY:MONTHLYREPORT:MONTHLYSWITCH:OFF")]
    MonthlySwitchOff,
} 