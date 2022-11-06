use std::fmt;

#[derive(Debug, Clone)]
pub enum RedisSetName{
    ValidUserId,
}

impl fmt::Display for RedisSetName{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedisSetName::ValidUserId => write!(f, "ValidUserId"),
        }
    }
}