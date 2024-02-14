
use std::net::Ipv4Addr;

static  CONFIG_PATH: &'static str = ".config.yml";

pub struct Config
{
    pub default_ip: Ipv4Addr
}

impl Config
{
   pub fn load() -> Self {
        Config{ default_ip: Ipv4Addr::new(192, 168, 100, 115)}
    }
}