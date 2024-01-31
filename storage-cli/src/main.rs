mod lib;
mod config;
use std::{array::from_mut, net::Ipv4Addr, str::FromStr};
use lib::{FileStruct, Info, MemoryInfo};
use clap::{Parser, Subcommand};
use config::Config;
use serde_json::de;

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
        ip_address:String,

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
    Ls
    {
        #[arg(short, long)]
        ip_address:Option<String>,
        path:String
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

fn print_directory(files:&Vec<&FileStruct>, previous:&str )
{
    for i in 0..files.len()
    {
        print!("{}", previous);
        let f = files[i];
        if i == files.len()-1
            {
                print!("└──");
            }
            else {
                print!("├──");
            }
        
        if f.is_directory
        {
           println!("{}/", f.name);
           let next;
           if i != files.len()-1
           {
             next = format!("{}│   ", previous);
           }
           else {
            next = format!("{}   ", previous);
           }
            print_directory(&f.files.iter().map(|x| x.as_ref()).collect(), &next);
        }
        else {
            println!("{}", f.name);
        }
    }
}

fn print_ls(files:&Vec<FileStruct>)
{
    println!("{0: <40} | {1: <10} | {2: <30} | {3: <30} | {4: <10}",
    "name","size kb", "creation date", "last modificated", "directory");
    for i in files
    {
        println!("{0: <40} | {1: <10} | {2: <30} | {3: <30} | {4: <10}", 
        i.name, i.size/1024, i.creation_date, i.last_modificated, i.is_directory);
    }
}

fn main() {
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

                    info = lib::info(&ip);
                }
                else
                {
                    println!("ip address incorect");
                    return;
                }
            } 
            else 
            {
                info = lib::info(&config.default_ip);
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

                    info = lib::memory(&ip);
                }
                else
                {
                    println!("ip address incorect");
                    return;
                }
            } 
            else 
            {
                info = lib::memory(&config.default_ip);
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
                if let Ok(ip) = Ipv4Addr::from_str(ip_address)
                {

                    let res = lib::post_config_from_file(&ip, &config);
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

            match lib::tree(&ip, &path, depth)
            {
                Ok(res) =>
                {
                    print_directory(&res.iter().map(|x| x).collect(), "");
                }
                Err(e) => println!("{}", e)
            }

        },
        Some(Commands::Ls { ip_address, path }) =>
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
            match lib::tree(&ip, &path, 0) {

                Ok(res) =>
                {
                    print_ls(&res);
                },
                Err(e) => println!("{}", e)
                
            }
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
                    match lib::post_file(&ip, local_path, device_path) {
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
                FileCommands::Mv {from, to} =>
                {
                    match lib::move_file(&ip, from, to)
                    {
                        Ok(()) => println!("file moved"),
                        Err(e) => println!("{}",e)
                    }
                },
                FileCommands::Rm {path} =>
                {
                    match lib::delete_file(&ip, path)
                    {
                        Ok(()) => println!("file deleted"),
                        Err(e) => println!("{}",e)
                    }
                }
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
