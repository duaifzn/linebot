use std::fmt;

pub enum LineBotProcess{
    ValidUserId,
}

impl fmt::Display for LineBotProcess{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBotProcess::ValidUserId => write!(f, "validUserId"),
        }
    }
}