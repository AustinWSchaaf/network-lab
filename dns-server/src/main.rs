mod net;
mod dns;
mod filter;
mod cache;
fn main() -> std::io::Result<()> {
    net::udp::run()
}