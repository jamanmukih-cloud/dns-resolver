pub struct DnsResolver {
    cache: Cache,
}

struct Cache;

impl DnsResolver {
    pub fn new() -> Self {
        Self { cache: Cache }
    }
    
    pub fn resolve(&self, domain: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Ok(vec!["93.184.216.34".to_string()])
    }
}
