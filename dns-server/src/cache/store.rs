use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct CacheEntry {
    pub response: Vec<u8>,
    pub expires_at: Instant,
}

pub struct DnsCache {
    store: HashMap<(String, u16), CacheEntry>,
    default_ttl: Duration,
}

impl DnsCache {
    pub fn new(default_ttl_secs: u64) -> Self {
        Self {
            store: HashMap::new(),
            default_ttl: Duration::from_secs(default_ttl_secs),
        }
    }

    pub fn get(&mut self, domain: &str, qtype: u16) -> Option<Vec<u8>> {
        let key = (domain.to_string(), qtype);

        if let Some(entry) = self.store.get(&key) {
            if Instant::now() < entry.expires_at {
                return Some(entry.response.clone());
            }
        }
        self.store.remove(&key);
        None
    }

    pub fn insert_with_ttl(&mut self, 
        domain: String, 
        qtype: u16, 
        response: Vec<u8>,
        ttl_secs: u64
    ) {
        let key = (domain, qtype);

        let entry = CacheEntry {
            response,
            expires_at: Instant::now() + Duration::from_secs(ttl_secs),
        };
        self.store.insert(key, entry);
    }
}