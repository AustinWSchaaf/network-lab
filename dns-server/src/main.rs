mod net;

fn main() -> std::io::Result<()> {
    net::udp::run()
}