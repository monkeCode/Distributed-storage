mod lib;
mod config;
use std::{net::Ipv4Addr, str::FromStr};
use lib::{Info, MemoryInfo};
use tokio;
use clap::{Parser, Subcommand};
use config::Config;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args
{
    
    #[command(subcommand)]
    command:Option<Commands>

}

#[derive(Subcommand, Debug)]
enum Commands{
    ///get info about device
    Info{

        ///ip address of device
        #[arg(short,long)]
        ip_address:Option<String>
    },

    ///send json config to device
    Config 
    {
        ///ip address of device
        #[arg(short, long)]
        ip_address:Option<String>,

        ///config path
        config:String
    },
    ///get memory information about device
    Memory
    {
        ///ip address of device
        #[arg(short, long)]
        ip_address:Option<String>
    },

    Tree
    {
        ///ip address of device
        #[arg(short, long)]
        ip_address:Option<String>,

        ///maximum depth of tree default 5
        #[arg(short, long,)]
        depth:Option<u32>,

        ///root folder of tree
        path:Option<String>
    },
    File
    {
                
        #[arg(short, long)]
        ip_address:Option<String>,

        #[command(subcommand)]
        command:FileCommands,
    }
}

#[derive(Subcommand, Debug)]
enum FileCommands
{
    Post{

        local_path:String,

        device_path:String
    },
    Get {
        device_path:String,
        local_path:String,

    },
    Mv{
        from:String,
        to:String,
    },
    Rm{
        path:String,
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = Config::load();
    let command = &args.command;
    
    match command {
        Some(Commands::Info { ip_address }) => {
            let info:Result<Info, reqwest::Error >;

            if ip_address.is_some() 
            {
                if let Ok(ip) = Ipv4Addr::from_str(ip_address.as_ref().unwrap())
                {

                    info = lib::info(&ip).await;
                }
                else
                {
                    println!("ip address incorect");
                    return;
                }
            } 
            else 
            {
                info = lib::info(&config.default_ip).await;
            }
            match info {
                Ok(res) => println!("{:#?}", res),
                Err(e) => println!("{}",e )
                
            }
        },
        Some(Commands::Memory { ip_address }) => {

            let info:Result<MemoryInfo, reqwest::Error>;

            if ip_address.is_some() 
            {
                if let Ok(ip) = Ipv4Addr::from_str(ip_address.as_ref().unwrap())
                {

                    info = lib::memory(&ip).await;
                }
                else
                {
                    println!("ip address incorect");
                    return;
                }
            } 
            else 
            {
                info = lib::memory(&config.default_ip).await;
            }
            match info {
                Ok(res) => 
                {
                    
                    println!("total {} mb, free {} mb ({}%)", (res.total)/1024/1024, res.free/1024/1024, 100-(res.free/res.total*100))
                },
                Err(e) => println!("{}",e )
                
            }
        },
        Some(Commands::Config { ip_address, config }) =>
        {
            if ip_address.is_some()
            {
                if let Ok(ip) = Ipv4Addr::from_str(ip_address.as_ref().unwrap())
                {

                    let res = lib::post_config_from_file(&ip, &config).await;
                    if res.is_err()
                    {
                        println!("{}", res.unwrap_err());
                    }
                    else {
                        println!("config applied");
                    }
                }
                else
                {
                    println!("ip address incorect");
                    return;
                }
            }
            else {
                println!("ip not found");
                return;
            }
        },
        Some(Commands::Tree { ip_address, depth, path }) =>
        {
            let ip:Ipv4Addr;
            if let Ok(ip_ad) = Ipv4Addr::from_str(ip_address.as_ref().unwrap_or(&config.default_ip.to_string()))
            {
                ip = ip_ad;
            }
            else 
            {
                println!("ip address incorect");
                return;
            }
            let depth = depth.to_owned().unwrap_or(5);
            let path = path.to_owned().unwrap_or(String::from("/"));
        },
        Some(Commands::File { ip_address, command }) =>
        {
            let ip:Ipv4Addr;
            if let Ok(ip_ad) = Ipv4Addr::from_str(ip_address.as_ref().unwrap_or(&config.default_ip.to_string()))
            {
                ip = ip_ad;
            }
            else 
            {
                println!("ip address incorect");
                return;
            }

            match command {

                FileCommands::Post { local_path, device_path } =>
                {
                    match lib::post_file(&ip, local_path, device_path).await {
                        Ok(()) => {
                            println!("file sended");
                            return;
                        }
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    }
                },
                _ =>
                {
                    panic!("command not implemed");
                }
            }
        }
        None => {
            println!("Unknown command")
        }   
    };

}
