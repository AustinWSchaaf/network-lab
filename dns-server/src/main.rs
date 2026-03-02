mod net;
mod dns;
mod filter;
fn main() -> std::io::Result<()> {
    net::udp::run()
}