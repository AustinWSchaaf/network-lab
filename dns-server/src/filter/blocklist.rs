use std::collections::HashSet;
use std::io;
use std::fs;

pub struct Blocklist {
    exact: HashSet<String>,
    wildcards: Vec<String>,
}

impl Blocklist {
    pub fn load(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;

        let mut exact = HashSet::new();
        let mut wildcards = Vec::new();

        for line in content.lines() {
            let domain = line.trim();

            if domain.is_empty() || domain.starts_with('#') {
                continue;
            }

            if domain.starts_with("*.") {
                wildcards.push(domain[2..].to_string());
            } else {
                exact.insert(domain.to_string());
            }
        }
        Ok(Self {exact, wildcards})
    }

    pub fn is_blocked(&self, domain: &str) -> bool {
        if self.exact.contains(domain) {
            return true;
        }

        for suffix in &self.wildcards {
            if domain.ends_with(suffix) {
                return true;
            }
        }

        false
    }
}