
use std::{error::Error, fs, net::Ipv4Addr, str::FromStr};
use::reqwest;
use reqwest::{Body, Error as ReqError};
use serde_json;
use serde::Deserialize;
use reqwest::blocking::multipart;

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
    pub ip:Ipv4Addr,
    pub mac:String,
    pub device_type:DeviceType
}

#[derive(Debug, Deserialize)]
pub struct MemoryInfo
{
    pub total:u64,
    pub free:u64
}

pub fn info(addres: &Ipv4Addr) -> Result<Info, ReqError>
{
    let json= reqwest::blocking::get(String::from("http://") + &addres.to_string() + &String::from("/info"))?.text()?;
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

pub fn post_config_from_file(device_ip: &Ipv4Addr, config_path:&String) -> Result<(), Box<dyn Error>>
{
    let res = fs::read(config_path)?;

    let s = String::from_utf8(res);

    Ok(post_config(device_ip, s.unwrap().to_owned())?)
}

pub fn post_config(device_ip: &Ipv4Addr, config_json:String) -> Result<(), ReqError>
{

    let client = reqwest::blocking::Client::new();
    let _response = client.post(String::from("http://") + &device_ip.to_string() + &String::from("/config"))
    .body(reqwest::blocking::Body::from(config_json))
    .send()?;

    Ok(())
}

pub fn memory(addres: &Ipv4Addr) -> Result<MemoryInfo, ReqError>
{
    let json= reqwest::blocking::get(String::from("http://") + &addres.to_string() + &String::from("/mem"))?.text()?;
    let data: MemoryInfo = serde_json::from_str(&json).expect("Cant parse json");
    Ok(data)
}

pub fn post_file(addres: &Ipv4Addr, file_path:&String, end_path:&String) -> Result<(), Box<dyn Error>>
{
    //let file = tfs::File::open(file_path).await?;
    //let body = file_to_body(file);
    let client = reqwest::blocking::Client::new();

    //let some_file = multipart::Part::stream(body).mime_str("text/plain")?;

    let form = multipart::Form::new().file("file", file_path)?;

    let res = client.post(format!("http://{}/file?path={}", addres.to_string(), end_path))
        .multipart(form)
        .send()?;
    
    Ok(())
}

pub fn tree(addres: &Ipv4Addr, root: &String, depth:u32)
{

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
