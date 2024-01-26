mod lib;
use::std::env;
use std::{error::Error, net::Ipv4Addr, ops::Index, str::FromStr};
use tokio;

#[tokio::main]
async fn main() {
    let args:Vec<String> = env::args().collect();
    let command = args[1].as_str();
    for i in 0..args.len()
    {
        println!("{} {}",i, args[i]);
    }
    let res = match command {

        "info" => lib::info().await,

        "config" => {
            if args.contains(&String::from("-i"))
            {
                let index = args.iter().position(|x| x == "-i").unwrap();
                let ip: &String = &args[index+1];
                let path = args.last().unwrap();
                if let Ok(address) = Ipv4Addr::from_str(ip.as_str())
                {
                    let res = lib::post_config_from_file(address, path).await;
                    if res.is_err()
                    {
                        println!("error");
                        
                    }
                    return;
                }
                else {
                    println!("ip address incorect");
                    return;
                }
            }
            println!("ip not found");
            return;
        },

        _ => {
            println!("unknown command");
            return;
        }
        
    };
    match res {

        Ok(s) => println!("{:#?}", s),
        Err(e) => println!("error")
        
    }
}
