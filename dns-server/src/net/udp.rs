use std::net::UdpSocket;
use std::io;

use crate::dns::header::DnsHeader;
use crate::dns::question::DnsQuestion;
use crate::filter::blocklist::Blocklist;
use crate::cache::store::DnsCache;

pub fn run() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:2053")?;
    println!("DNS Server listening on 0.0.0.0:2053");

    let blocklist = Blocklist::load("blocklist.txt")?;
    println!("Blocklist loaded.");

    let mut cache = DnsCache::new(60); // 60 second default TTL
    println!("Cache initialized.");

    let mut buffer = [0u8; 512];

    loop {
        let (size, src) = socket.recv_from(&mut buffer)?;
        println!("\nReceived {} bytes from {}", size, src);

        let header = DnsHeader::parse(&buffer);
        let pos = 12;

        if header.qdcount == 0 {
            continue;
        }

        let (question, _) = DnsQuestion::parse(&buffer, pos);

        println!("Domain: {}", question.name);
        println!("Type: {}", question.qtype);

        // 🔴 Blocklist check
        if blocklist.is_blocked(&question.name) {
            println!("Blocked: {}", question.name);

            let response = build_nxdomain_response(&buffer[..size]);
            socket.send_to(&response, src)?;
            continue;
        }

        // 🟢 Cache lookup
        if let Some(cached_response) = cache.get(&question.name, question.qtype) {
            println!("Cache hit: {}", question.name);
            socket.send_to(&cached_response, src)?;
            continue;
        }

        // 🌍 Forward upstream
        let response = forward_to_upstream(&buffer[..size])?;

        println!("Cache insert: {}", question.name);
        cache.insert(
            question.name.clone(),
            question.qtype,
            response.clone(),
        );

        socket.send_to(&response, src)?;
    }
}

fn forward_to_upstream(request: &[u8]) -> io::Result<Vec<u8>> {
    let upstream = "8.8.8.8:53";
    let upstream_socket = UdpSocket::bind("0.0.0.0:0")?;

    upstream_socket.send_to(request, upstream)?;

    let mut response_buffer = [0u8; 512];
    let (size, _) = upstream_socket.recv_from(&mut response_buffer)?;

    Ok(response_buffer[..size].to_vec())
}

fn build_nxdomain_response(request: &[u8]) -> Vec<u8> {
    let mut response = request.to_vec();

    // Set QR bit (response flag)
    response[2] |= 0x80;

    // Clear RCODE bits
    response[3] &= 0xF0;

    // Set RCODE = 3 (NXDOMAIN)
    response[3] |= 0x03;

    // ANCOUNT = 0
    response[6] = 0;
    response[7] = 0;

    response
}