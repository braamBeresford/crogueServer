extern crate byteorder;

use byteorder::{LittleEndian, WriteBytesExt};
use std::mem;
use std::net::UdpSocket;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::io;

pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub id: i32
}

fn main() {
    let mut entities: Vec<Entity> = Vec::new();
    
    while true{
        let res = snd();
        break;
    }
}


fn snd() -> Result<(), io::Error> {
    let ip = Ipv4Addr::new(127, 0, 0, 1); 
    let connection = SocketAddrV4::new(ip, 9992);

    let socket = try!(UdpSocket::bind(connection));

    let connection2 = SocketAddrV4::new(ip, 9991);

    let test: u32 = 2;
    let bytes: [u8; 4] = unsafe {std::mem::transmute::<u32, [u8; 4]>(test.to_le())};
    let buf = &bytes;

        let i: i64 = 12345;
    let mut bs = [0u8; mem::size_of::<i64>()];
    bs.as_mut()
        .write_i64::<LittleEndian>(i)
        .expect("Unable to write");

    try!(socket.send_to(&bs, connection2));
    // println!("{:?}", buf);


    // for x in &bs {
    //     println!("{:X}", x);
    // }

    Ok(()) 
}


