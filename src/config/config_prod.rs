use crate::config::Config;
use std::env;
use std::borrow::Cow;

pub fn prod() ->Config<'static>{
    Config{
        access_token: Cow::Owned(env::var("ACCESS_TOKEN").expect(" load ACCESS_TOKEN env error!!")),
        secret: Cow::Owned(env::var("SECRET").expect(" load SECRET env error!!")),
        redis_url: Cow::Owned(env::var("REDIS_URL").expect(" load REDIS_URL env error!!")),
        mysql_url: Cow::Owned(env::var("MYSQL_URL").expect(" load MYSQL_URL env error!!")),
    }
}