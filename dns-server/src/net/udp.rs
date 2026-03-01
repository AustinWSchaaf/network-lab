use std::net::UdpSocket;
use std::io;

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
    }
}