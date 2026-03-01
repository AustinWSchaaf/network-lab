
pub struct DnsHeader {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DnsHeader {
    pub fn parse(buffer: &[u8]) -> Self {
        Self {
            id: u16::from_be_bytes([buffer[0], buffer[1]]),
            flags: u16::from_be_bytes([buffer[2], buffer[3]]),
            qdcount: u16::from_be_bytes([buffer[4], buffer[5]]),
            ancount: u16::from_be_bytes([buffer[6], buffer[7]]),
            nscount: u16::from_be_bytes([buffer[8], buffer[9]]),
            arcount: u16::from_be_bytes([buffer[10], buffer[11]]),
        }
    }
}