mod lib;
use::std::env;
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
        _ => panic!("unknown command")
        
    };
    match res {

        Ok(s) => println!("{}", s),
        Err(e) => println!("error")
        
    }
}
