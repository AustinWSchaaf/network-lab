use std::net::UdpSocket;
use std::io;
use crate::dns::header::DnsHeader;
use crate::dns::question::DnsQuestion;
use crate::filter::blocklist::Blocklist;


const BLOCKED_DOMAINS: &[&str] = &[
    "badsite.com",
    "ads.google.com",
];

pub fn run() -> io::Result<()> {
    let blocklist = Blocklist::load("blocklist.txt")?;
    println!("Blocklist loaded.");

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
        let mut pos = 12;

        if header.qdcount > 0 {
            let (question, _) = DnsQuestion::parse(&buffer, pos);
            println!("Domain: {}", question.name);

            if blocklist.is_blocked(&question.name) {
                println!("Blocked: {}", question.name);

                let response = build_nxdomain_repsonse(&buffer[..size]);
                socket.send_to(&response, src)?;
                continue;
            }
        }

        //Forward request upstream
        let response = forward_to_upstream(&buffer[..size])?;

        //Send response back to original client
        socket.send_to(&response, src)?;
    }
}

fn forward_to_upstream(request: &[u8]) -> io::Result<(Vec<u8>)> {
    let upstream = "8.8.8.8:53";
    let upstream_socket = UdpSocket::bind("0.0.0.0:0")?;

    upstream_socket.send_to(request, upstream)?;

    let mut response_buffer = [0u8; 512];
    let (size, _) = upstream_socket.recv_from(&mut response_buffer)?;
    Ok(response_buffer[..size].to_vec())
}

fn build_nxdomain_repsonse(request: &[u8]) -> Vec<u8> {
    let mut response = request.to_vec();
    
    // Set QR bit (response)
    response[2] &= 0x80;

    // Clear RCODE bits
    response[3] &= 0xF0;

    // ANCOUNT = 0
    response[6] = 0;
    response[7] = 0;

    response
}