mod net;
mod dns;

fn main() -> std::io::Result<()> {
    net::udp::run()
}