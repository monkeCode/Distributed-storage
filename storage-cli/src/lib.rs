
use std::{error::Error, net::Ipv4Addr, str::FromStr};
use::reqwest;
use reqwest::{Body, Error as reqError, StatusCode};
use serde_json;
use std::fs;

static mut IP:Ipv4Addr = Ipv4Addr::new(192, 168, 100, 220);

#[derive(Debug)]
pub enum DeviceType {
    Node,
    Master
}

impl DeviceType {
    pub fn from_str(s: &str) -> DeviceType
    {
        match s
        {
            "node" => DeviceType::Node,
            "master" => DeviceType::Master,
            _ => panic!("device type not found")
        }
    }

}

#[derive(Debug)]
pub struct Info
{
    ip:Ipv4Addr,
    mac:String,
    device_type:DeviceType
}

pub async fn info() -> Result<Info, reqError>
{
    unsafe{
        let json= reqwest::get(String::from("http://") + &IP.to_string() + &String::from("/info")).await?.text().await?;
        let data: serde_json::Value = serde_json::from_str(&json).expect("Cant parse json");

        let ip_str: &String = match &data["ip"]{
            serde_json::Value::String(s) => s,
            _ => panic!("ip wrong type")
        };
        let ip = Ipv4Addr::from_str(&ip_str).unwrap();
        let mac = match &data["mac"] {
            serde_json::Value::String(s) => s,
            _ => panic!("mac wrong type")
        };
        let d_type = match &data["type"] {
            serde_json::Value::String(s) => DeviceType::from_str(&s),
            _ => panic!("device wrong type")
        };
        return Ok(Info{ip:ip, mac:mac.to_owned(), device_type:d_type});
    }
}

pub async fn post_config_from_file(device_ip:Ipv4Addr, config_path:&String) -> Result<bool, Box<dyn Error>>
{
    let res = match fs::read(config_path)
    {
        Ok(data) => data,
        Err(e) => return Err(Box::new(e))
    };

    let s = String::from_utf8(res);

    let r = post_config(device_ip, s.unwrap().to_owned()).await;
    match r {

        Ok(b) => Ok(b),
        Err(e) => Err(e)
        
    }
}

pub async fn post_config(device_ip:Ipv4Addr, config_json:String) -> Result<bool, Box<dyn Error>>
{
    unsafe
    {
        let client = reqwest::Client::new();
        let responce = client.post(String::from("http://") + &IP.to_string() + &String::from("/config")).body(Body::from(config_json)).send()
        .await?;
        Ok(responce.status() == StatusCode::OK)
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
