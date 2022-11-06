use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum LineBotProcess{
    Hello,
    PartTimer,
    PeriodOfOperationReport,
    SumOfOperationReport,
    Presentation,
}

impl fmt::Display for LineBotProcess{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBotProcess::Hello => write!(f, "安安"),
            LineBotProcess::PeriodOfOperationReport => write!(f, "定期營運報表"),
            LineBotProcess::SumOfOperationReport => write!(f, "累計營運報表"),
            LineBotProcess::Presentation => write!(f, "成果展報表"),
            LineBotProcess::PartTimer => write!(f, "工讀生"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PeriodOfOperationReport{
    Own,
    DayOfRewardPoint,
    TrackOfSendRewardPoint,
    StatusOfRewardPoint,
    OrderTrend,
    RevenueOfRewardPoint,
    SalesChartsTop10,
    WeekOfSalesChartsTop10,
    DayOfTrafficAnalysis,
    TotalOfOrderAnalysis,
    DayOfCustomerServiceAnalysis,
    DayOfClassOfCustomerServiceAnalysis,
    SendRewardPointTable,
    UseRewardPointTable,
    RevokeRewardPointTable,
}

impl fmt::Display for PeriodOfOperationReport{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeriodOfOperationReport::Own => write!(f, "定期營運報表"),
            PeriodOfOperationReport::DayOfRewardPoint => write!(f, "每日點數營運數量統計"),
            PeriodOfOperationReport::TrackOfSendRewardPoint => write!(f, "點數發放進度追蹤與趨勢"),
            PeriodOfOperationReport::StatusOfRewardPoint => write!(f, "點數發放與使用狀況"),
            PeriodOfOperationReport::OrderTrend => write!(f, "訂單趨勢"),
            PeriodOfOperationReport::RevenueOfRewardPoint => write!(f, "累計營運數字"),
            PeriodOfOperationReport::SalesChartsTop10 => write!(f, "累計銷售排行榜Top10"),
            PeriodOfOperationReport::WeekOfSalesChartsTop10 => write!(f, "週銷售排行榜Top10"),
            PeriodOfOperationReport::DayOfTrafficAnalysis => write!(f, "每日網站流量數據分析"),
            PeriodOfOperationReport::TotalOfOrderAnalysis => write!(f, "累計數位支付訂單分析"),
            PeriodOfOperationReport::DayOfCustomerServiceAnalysis => write!(f, "每日客服通話與服務量統計"),
            PeriodOfOperationReport::DayOfClassOfCustomerServiceAnalysis => write!(f, "每日客服來電類別服務量統計"),
            PeriodOfOperationReport::SendRewardPointTable => write!(f, "點數管控表-發放情形"),
            PeriodOfOperationReport::UseRewardPointTable => write!(f, "點數管控表-使用情形"),
            PeriodOfOperationReport::RevokeRewardPointTable => write!(f, "點數管控表-核銷情形"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SumOfOperationReport{
    Own,
    SumOfRewardPoint,
    SumOfRevenue,
    SalesCharts,
    SubsidyFirm,
    CustomerServiceStatic
}

impl fmt::Display for SumOfOperationReport{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SumOfOperationReport::Own => write!(f, "累計營運報表"),
            SumOfOperationReport::SumOfRewardPoint => write!(f, "累計點數營運數量統計"),
            SumOfOperationReport::SumOfRevenue => write!(f, "累計銷售情形"),
            SumOfOperationReport::SalesCharts => write!(f, "銷售排行榜"),
            SumOfOperationReport::SubsidyFirm => write!(f, "受補助企業分布"),
            SumOfOperationReport::CustomerServiceStatic => write!(f, "客服及諮詢服務狀況"),
        }
    }
}