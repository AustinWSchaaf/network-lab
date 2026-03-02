pub fn extract_min_ttl(response: &[u8]) -> Option<u32> {
    if response.len() < 12 {
        return None;
    }

    let ancount = u16::from_be_bytes([response[6], response[7]]) as usize;
    if ancount == 0 {
        return None;
    }

    let mut pos = 12;

    loop {
        if pos >= response.len() {
            return None;
        }

        let len = response[pos] as usize;
        pos += 1;

        if len == 0 {
            break;
        }

        pos += len;
    }

    pos += 4;

    let mut min_ttl: Option<u32> = None;

    for _ in 0..ancount {
        if pos + 10 >= response.len() {
            return None;
        }

        pos += 2;
        
        pos += 4;

        let ttl = u32::from_be_bytes([
            response[pos],
            response[pos + 1],
            response[pos + 2],
            response[pos + 3],
        ]);
        pos += 4;

        // Track minimum TTL
        min_ttl = Some(match min_ttl {
            Some(current_min) => current_min.min(ttl),
            None => ttl,
        });

        let rdlength = u16::from_be_bytes([response[pos], response[pos + 1]]) as usize;
        pos += 2;

        pos += rdlength;
    }

    min_ttl
}