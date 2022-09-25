use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum LineBotProcess{
    Hello,
    PartTimer,
    OperationReport,
    SalesCharts,
}

impl fmt::Display for LineBotProcess{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBotProcess::Hello => write!(f, "安安"),
            LineBotProcess::PartTimer => write!(f, "工讀生"),
            LineBotProcess::OperationReport => write!(f, "營運報表"),
            LineBotProcess::SalesCharts => write!(f, "銷售排行榜"),
        }
    }
}