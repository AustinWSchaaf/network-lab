use std::net::UdpSocket;
use std::io;

use crate::dns::header::DnsHeader;
use crate::dns::question::DnsQuestion;

pub fn run() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:2053")?;
    println!("DNS Echo Server Listening on 0.0.0.0:2053");

    let mut buffer = [0u8; 512];

    loop {
        let (size, src) = socket.recv_from(&mut buffer)?;
        println!("Received {} bytes from {}", size, src);

        println!("Raw packer: {:x?}", &buffer[..size]);

        socket.send_to(&buffer[..size], src)?;
        println!("Echoed packet back {}\n", src);

        let header = DnsHeader::parse(&buffer);        
        println!("Questions: {}", header.qdcount);

        let mut pos = 12; //DNS Header is 12 bytes

        if header.qdcount > 0 {
            let (question, _) = DnsQuestion::parse(&buffer, pos);

            println!("Domain: {}", question.name);
            println!("Type: {}", question.qtype);
        }
    }
}