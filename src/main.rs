mod dns;

use std::net::UdpSocket;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let listener = UdpSocket::bind("127.0.0.1:53")?;
    
    Ok(())
}
