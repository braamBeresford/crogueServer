use std::net::UdpSocket;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::io;

pub struct Entity {
    pub x: u8,
    pub y: u8
}

fn main() {
    let mut entities: Vec<Entity> = Vec::new();
    
    while true{
        let res = snd();
    }
}


fn snd() -> Result<(), io::Error> {
    let ip = Ipv4Addr::new(127, 0, 0, 1); 
    let connection = SocketAddrV4::new(ip, 9992);

    let socket = try!(UdpSocket::bind(connection));

    let connection2 = SocketAddrV4::new(ip, 9991);

    let test = 69;
    let buf = &[test, 0x02, 0x03];
    try!(socket.send_to(buf, connection2));
    println!("{:?}", buf);

    Ok(()) 
}


