pub mod config_dev;
pub mod config_prod;
use crate::config::config_dev::dev;
use crate::config::config_prod::prod;
use std::borrow::Cow;
use std::env;

pub struct Config<'a> {
    pub access_token: Cow<'a ,str>,
    pub secret: Cow<'a ,str>,
}

impl Config<'static> {
    pub fn load() -> Self {
        let args: Vec<String> = env::args().collect();
        match args.len() {
            0 | 1 => return dev(),
            _ => {}
        }
        let env = args[1].as_str();
        match env{
            "dev" => dev(),
            "prod" => prod(),
            _ => dev()
        }
    }
}
