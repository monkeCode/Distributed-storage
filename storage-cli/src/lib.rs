
use std::net::Ipv4Addr;
use::reqwest;
use reqwest::Error;

static mut IP:Ipv4Addr = Ipv4Addr::new(192, 168, 100, 215);


pub async fn info() -> Result<String, Error>
{
    unsafe{
        let res = reqwest::get(String::from("http://") + &IP.to_string() + &String::from("/info")).await?.text().await?;
        return Ok(res);
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
