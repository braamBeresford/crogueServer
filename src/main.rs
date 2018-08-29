use std::net::UdpSocket;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::io;
fn main() {
    
       while true{
           let res = recv();
           println!("{:?}", res);
       }
       
}


fn recv() -> Result<(), io::Error> { // Define the local connection information 
    let ip = Ipv4Addr::new(127, 0, 0, 1); 
    let connection = SocketAddrV4::new(ip, 9991);

    // Bind the socket
    let socket = try!(UdpSocket::bind(connection));

    // Read from the socket
    let mut buf = [0; 10];
    let (amt, src) = try!(socket.recv_from(&mut buf));

    // Print only the valid data (slice)
    println!("{:?}", &buf[0 .. amt]);

    Ok(()) 
}

