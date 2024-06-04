use std::net::{AddrParseError, IpAddr};
fn main() {
    let localhost_v4: Result<IpAddr, AddrParseError> = "127.00.1".parse();

    match localhost_v4{
        Ok(v4) => println!("The Ip Address is {v4}"),
         Err(error) => println!("Not a valid ip address {error}")
    }

}
