
pub struct DnsQuestion {
    pub name: String,
    pub qtype: u16, 
    pub qclass: u16,
}

impl DnsQuestion {
    pub fn parse(buffer: &[u8], mut pos: usize) -> (Self, usize) {
        let mut labels = Vec::new();

        loop {
            let len = buffer[pos] as usize;
            pos += 1;

            if len == 0 {
                break;
            }

            let label = &buffer[pos..pos + len];
            labels.push(String::from_utf8_lossy(label).to_string());
            pos += len;
        }

        let name = labels.join(".");

        let qtype = u16::from_be_bytes([buffer[pos], buffer[pos + 1]]);
        pos += 2;

        let qclass = u16::from_be_bytes([buffer[pos], buffer[pos + 1]]);
        pos += 2;

        (
            Self {
                name,
                qtype,
                qclass,
            },
            pos,
        )
    }
}