use crate::config::Config;
use std::env;
use std::borrow::Cow;

pub fn prod() ->Config<'static>{
    Config{
        access_token: Cow::Owned(env::var("ACCESS_TOKEN").expect(" load env error!!")),
        secret: Cow::Owned(env::var("SECRET").expect(" load env error!!")),
    }
}