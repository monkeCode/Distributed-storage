
use std::{error::Error, fs, io::{self, Cursor, Read, Write}, net::Ipv4Addr, str::FromStr};
use::reqwest;
use reqwest::{Error as ReqError};
use serde_json;
use serde::Deserialize;
use reqwest::blocking::multipart;
use chrono::{DateTime, Local, TimeZone};

#[derive(Debug)]
pub enum DeviceType {
    Node,
    Master,
}

impl DeviceType {
    pub fn from_str(s: &str) -> DeviceType {
        match s {
            "node" => DeviceType::Node,
            "master" => DeviceType::Master,
            _ => panic!("device type not found"),
        }
    }
}

#[derive(Debug)]
pub struct FileStruct
{
    pub is_directory:bool,
    pub name:String,
    pub last_modificated:DateTime<Local>,
    pub creation_date:DateTime<Local>,
    pub size:i64,
    pub files: Vec<Box<FileStruct>>
}

impl FileStruct {

    fn from_json(val:&serde_json::Value) -> Self
    {
        let is_directory = val["isDirectory"].as_bool().expect("bad type isDirectory");
        let name = String::from(val["name"].as_str().expect("bad type name"));
        let last_modificated = Local.timestamp_millis_opt(val["lastModificated"].as_i64().expect("bad type lastModificated"))
        .single().expect("cannot convert to datetime");
        let creation_date =  Local.timestamp_millis_opt(val["creationDate"].as_i64().expect("bad type creationDate"))
        .single().expect("cannot convert to datetime");
        let size: i64 =  val["size"].as_i64().expect("bad type size");
        if is_directory
        {
            let mut files = Vec::<Box<FileStruct>>::new();
            for f in val["files"].as_array().expect("bad type files")
            {
                files.push(Box::new(FileStruct::from_json(f)));
            }
            return FileStruct{is_directory, name, last_modificated, creation_date, files, size}
        }
        return FileStruct{is_directory, name, creation_date, size, last_modificated, files:Vec::new()}
    
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

pub fn info(address: &Ipv4Addr) -> Result<Info, ReqError>
{
    let json= reqwest::blocking::get(String::from("http://") + &address.to_string() + &String::from("/info"))?.text()?;
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

pub fn post_config_from_file(address: &Ipv4Addr, config_path:&String) -> Result<(), Box<dyn Error>>
{
    let res = fs::read(config_path)?;

    let s = String::from_utf8(res);

    Ok(post_config(address, s.unwrap().to_owned())?)
}

pub fn post_config(address: &Ipv4Addr, config_json:String) -> Result<(), ReqError>
{

    let client = reqwest::blocking::Client::new();
    client.post(String::from("http://") + &address.to_string() + &String::from("/config"))
    .body(reqwest::blocking::Body::from(config_json))
    .send()?;

    Ok(())
}

pub fn memory(address: &Ipv4Addr) -> Result<MemoryInfo, ReqError>
{
    let json= reqwest::blocking::get(String::from("http://") + &address.to_string() + &String::from("/mem"))?.text()?;
    let data: MemoryInfo = serde_json::from_str(&json).expect("Cant parse json");
    Ok(data)
}

pub fn post_file(address: &Ipv4Addr, file_path:&String, end_path:&String) -> Result<(), Box<dyn Error>>
{
    let client = reqwest::blocking::Client::new();
    let form = multipart::Form::new().file("file", file_path)?;
    client.post(format!("http://{}/file?path={}", address.to_string(), end_path))
        .multipart(form)
        .send()?;
    
    Ok(())
}



pub fn tree(address: &Ipv4Addr, root: &String, depth:u32) -> Result<Vec<FileStruct>, Box<dyn Error>>
{
    let json= reqwest::blocking::get( format!("http://{}/tree?depth={}&path={}", 
    address.to_string(), depth, root))?
    .text()?;
    let data: Vec<serde_json::Value>;
    if let serde_json::Value::Array(r) = serde_json::from_str(&json).expect("Cant parse json")
    {
        data = r;
    }
    else {
        panic!("empty result");
    }
    let mut res_vec = Vec::new();
    for i in data
    {
        res_vec.push(FileStruct::from_json(&i))
    }
    Ok(res_vec)
}

pub fn move_file(address: &Ipv4Addr, from:&String, to:&String) -> Result<(), ReqError>
{
    let client = reqwest::blocking::Client::new();

    client.put(format!("http://{}/file?path={}&new path={}", address.to_string(), from, to))
        .send()?;
    Ok(())
}

pub fn delete_file(address: &Ipv4Addr, file_path:&String, ) -> Result<(), ReqError>
{
    let client = reqwest::blocking::Client::new();

    client.delete(format!("http://{}/file?path={}", address.to_string(), file_path))
        .send()?;
    Ok(())
}

pub fn get_file_from_node(address: &Ipv4Addr, from_file:&String, to:&String) -> Result<(), Box<dyn Error>>
{
    let client = reqwest::blocking::Client::new();
    let res = client.get(format!("http://{}/file?path={}", address.to_string(), from_file))
        .send()?.bytes()?;

    let mut file = fs::File::create(to)?;
    let mut content = Cursor::new(res);
    io::copy(&mut content, &mut file)?;
    Ok(())
}
pub fn get_file(address: &Ipv4Addr, from_file:&String, to:&String)-> Result<(), Box<dyn Error>>
{
    get_file_from_node(address, from_file, to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
