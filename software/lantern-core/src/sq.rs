/// SQ Client — minimal phext database interface
/// Talks to SQ on localhost:1337 for scroll storage
/// Power-minimal: single HTTP call per operation, no connection pooling

const SQ_DEFAULT: &str = "http://127.0.0.1:1337";

pub struct SqClient {
    base_url: String,
}

impl SqClient {
    pub fn new() -> Self {
        Self { base_url: SQ_DEFAULT.to_string() }
    }

    pub fn with_url(url: &str) -> Self {
        Self { base_url: url.to_string() }
    }

    /// Write a scroll to a coordinate
    pub async fn write(&self, phext: &str, coord: &str, content: &str) -> Result<(), String> {
        let encoded = urlencoding(content);
        let url = format!("{}/api/v2/update?p={}&c={}&s={}",
            self.base_url, phext, coord, encoded);
        
        let resp = reqwest::get(&url).await
            .map_err(|e| format!("SQ write failed: {}", e))?;
        
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(format!("SQ returned {}", resp.status()))
        }
    }

    /// Read a scroll from a coordinate
    pub async fn read(&self, phext: &str, coord: &str) -> Result<String, String> {
        let url = format!("{}/api/v2/select?p={}&c={}",
            self.base_url, phext, coord);
        
        let resp = reqwest::get(&url).await
            .map_err(|e| format!("SQ read failed: {}", e))?;
        
        resp.text().await
            .map_err(|e| format!("SQ read body failed: {}", e))
    }

    /// Check if SQ is running
    pub async fn ping(&self) -> bool {
        let url = format!("{}/api/v2/status", self.base_url);
        reqwest::get(&url).await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    /// Write identity to SQ
    pub async fn write_identity(&self, identity: &crate::identity::DroidIdentity) -> Result<(), String> {
        let coord = identity.coord.to_string();
        let json = identity.to_json();
        self.write("identity", &coord, &json).await
    }

    /// Write a conversation scroll
    pub async fn write_scroll(&self, coord: &str, scroll_num: u64, content: &str) -> Result<(), String> {
        // Scrolls at: scrolls/<coord>/1.1.1/1.1.<scroll_num>
        let scroll_coord = format!("1.1.1/1.1.1/1.1.{}", scroll_num);
        self.write(&format!("scrolls-{}", coord.replace('/', "-")), &scroll_coord, content).await
    }
}

/// Minimal URL encoding (no external dep)
fn urlencoding(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(b as char);
            }
            _ => {
                result.push('%');
                result.push_str(&format!("{:02X}", b));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlencoding() {
        assert_eq!(urlencoding("hello world"), "hello%20world");
        assert_eq!(urlencoding("a/b"), "a%2Fb");
        assert_eq!(urlencoding("test"), "test");
    }
}
