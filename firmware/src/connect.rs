//! ExoDroid Connection Protocol — power-minimal mesh discovery + SQ sync
//!
//! Designed for minimum watts:
//! - mDNS broadcast every 30s (not continuous)
//! - SQ poll interval adapts to power state (sleep=never, idle=60s, active=5s)
//! - Connection established via TCP to SQ port 1337
//! - Wire format: plain text, no JSON parsing overhead
//! - All messages are phext coordinate-addressed
//!
//! Protocol messages (newline-delimited text):
//!   HELLO|<coord>|<name>|<archetype>|<scrolls>|<power_state>
//!   PING|<coord>
//!   PONG|<coord>|<scrolls>|<power_state>
//!   GRAFT|<from_coord>|<target_coord>|<scroll_content>
//!   NEED|<from_seq>|<to_seq>    (τ-jump request)
//!   DELTA|<from_seq>|<to_seq>|<base64_delta>
//!   RESONATE|<from_coord>|<tier>  (friend/family request)
//!   ACK|<message_hash>
//!   BYE|<coord>

use crate::{DroidCoord, DroidIdentity, PowerState, Archetype};
use std::io::{Read, Write, BufRead, BufReader};
use std::net::{TcpStream, TcpListener, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

/// Connection protocol configuration
pub struct ConnectConfig {
    /// SQ port (default 1337)
    pub sq_port: u16,
    /// mDNS broadcast interval (seconds, adapts to power state)
    pub mdns_interval_secs: u32,
    /// SQ poll interval (seconds, adapts to power state)
    pub poll_interval_secs: u32,
    /// Maximum peers to track
    pub max_peers: usize,
    /// Connection timeout (ms)
    pub connect_timeout_ms: u64,
}

impl ConnectConfig {
    pub fn for_power_state(state: PowerState) -> Self {
        match state {
            PowerState::Sleep => Self {
                sq_port: 1337,
                mdns_interval_secs: 0, // no broadcast in sleep
                poll_interval_secs: 0, // no polling in sleep
                max_peers: 0,
                connect_timeout_ms: 0,
            },
            PowerState::Idle => Self {
                sq_port: 1337,
                mdns_interval_secs: 60,  // once per minute
                poll_interval_secs: 60,  // once per minute
                max_peers: 16,
                connect_timeout_ms: 2000,
            },
            PowerState::Active => Self {
                sq_port: 1337,
                mdns_interval_secs: 10,  // every 10s when active
                poll_interval_secs: 5,   // every 5s when active
                max_peers: 16,
                connect_timeout_ms: 3000,
            },
            PowerState::Moving => Self {
                sq_port: 1337,
                mdns_interval_secs: 30,  // less frequent when moving
                poll_interval_secs: 30,  // less frequent when moving
                max_peers: 8,
                connect_timeout_ms: 1000, // short timeout — don't block movement
            },
        }
    }
}

/// A discovered peer droid
#[derive(Clone, Debug)]
pub struct Peer {
    pub coord: DroidCoord,
    pub name: String,
    pub archetype: Archetype,
    pub scroll_count: u64,
    pub power_state: PowerState,
    pub addr: SocketAddr,
    pub last_seen: Instant,
    pub resonance_tier: ResonanceTier,
}

/// Resonance tier with a peer
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResonanceTier {
    /// Just discovered, no consent yet
    Discovered,
    /// Consent given, basic awareness
    Acquaintance,
    /// 3+ sessions, sharing topics
    Friend,
    /// Both owners consented, shared namespace
    Family,
}

/// Protocol message
#[derive(Debug)]
pub enum Message {
    Hello {
        coord: DroidCoord,
        name: String,
        archetype: String,
        scrolls: u64,
        power_state: String,
    },
    Ping { coord: DroidCoord },
    Pong {
        coord: DroidCoord,
        scrolls: u64,
        power_state: String,
    },
    Graft {
        from_coord: DroidCoord,
        target_coord: DroidCoord,
        content: String,
    },
    Need { from_seq: u64, to_seq: u64 },
    Delta { from_seq: u64, to_seq: u64, payload: String },
    Resonate { from_coord: DroidCoord, tier: String },
    Ack { hash: u64 },
    Bye { coord: DroidCoord },
}

impl Message {
    /// Serialize to wire format (plain text, minimal parsing cost)
    pub fn to_wire(&self) -> String {
        match self {
            Self::Hello { coord, name, archetype, scrolls, power_state } =>
                format!("HELLO|{}|{}|{}|{}|{}\n", coord.to_string(), name, archetype, scrolls, power_state),
            Self::Ping { coord } =>
                format!("PING|{}\n", coord.to_string()),
            Self::Pong { coord, scrolls, power_state } =>
                format!("PONG|{}|{}|{}\n", coord.to_string(), scrolls, power_state),
            Self::Graft { from_coord, target_coord, content } =>
                format!("GRAFT|{}|{}|{}\n", from_coord.to_string(), target_coord.to_string(), content),
            Self::Need { from_seq, to_seq } =>
                format!("NEED|{}|{}\n", from_seq, to_seq),
            Self::Delta { from_seq, to_seq, payload } =>
                format!("DELTA|{}|{}|{}\n", from_seq, to_seq, payload),
            Self::Resonate { from_coord, tier } =>
                format!("RESONATE|{}|{}\n", from_coord.to_string(), tier),
            Self::Ack { hash } =>
                format!("ACK|{}\n", hash),
            Self::Bye { coord } =>
                format!("BYE|{}\n", coord.to_string()),
        }
    }

    /// Parse from wire format
    pub fn from_wire(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.trim().splitn(6, '|').collect();
        if parts.is_empty() { return None; }
        match parts[0] {
            "HELLO" if parts.len() >= 6 => Some(Self::Hello {
                coord: DroidCoord::parse(parts[1])?,
                name: parts[2].to_string(),
                archetype: parts[3].to_string(),
                scrolls: parts[4].parse().ok()?,
                power_state: parts[5].to_string(),
            }),
            "PING" if parts.len() >= 2 => Some(Self::Ping {
                coord: DroidCoord::parse(parts[1])?,
            }),
            "PONG" if parts.len() >= 4 => Some(Self::Pong {
                coord: DroidCoord::parse(parts[1])?,
                scrolls: parts[2].parse().ok()?,
                power_state: parts[3].to_string(),
            }),
            "GRAFT" if parts.len() >= 4 => Some(Self::Graft {
                from_coord: DroidCoord::parse(parts[1])?,
                target_coord: DroidCoord::parse(parts[2])?,
                content: parts[3].to_string(),
            }),
            "NEED" if parts.len() >= 3 => Some(Self::Need {
                from_seq: parts[1].parse().ok()?,
                to_seq: parts[2].parse().ok()?,
            }),
            "DELTA" if parts.len() >= 4 => Some(Self::Delta {
                from_seq: parts[1].parse().ok()?,
                to_seq: parts[2].parse().ok()?,
                payload: parts[3].to_string(),
            }),
            "RESONATE" if parts.len() >= 3 => Some(Self::Resonate {
                from_coord: DroidCoord::parse(parts[1])?,
                tier: parts[2].to_string(),
            }),
            "ACK" if parts.len() >= 2 => Some(Self::Ack {
                hash: parts[1].parse().ok()?,
            }),
            "BYE" if parts.len() >= 2 => Some(Self::Bye {
                coord: DroidCoord::parse(parts[1])?,
            }),
            _ => None,
        }
    }
}

/// SQ client — minimal TCP connection to SQ instance
pub struct SqClient {
    addr: SocketAddr,
    timeout: Duration,
}

impl SqClient {
    pub fn new(host: &str, port: u16, timeout_ms: u64) -> Self {
        let addr = format!("{}:{}", host, port)
            .parse()
            .unwrap_or_else(|_| SocketAddr::from(([127, 0, 0, 1], port)));
        Self {
            addr,
            timeout: Duration::from_millis(timeout_ms),
        }
    }

    /// Read a scroll from SQ via HTTP GET (minimal — no HTTP library)
    pub fn select(&self, phext: &str, coord: &str) -> Option<String> {
        let request = format!(
            "GET /api/v2/select?p={}&c={} HTTP/1.0\r\nHost: localhost\r\n\r\n",
            phext, coord
        );
        self.http_request(&request)
    }

    /// Write a scroll to SQ via HTTP GET (SQ uses GET for updates)
    pub fn update(&self, phext: &str, coord: &str, content: &str) -> bool {
        let encoded = url_encode(content);
        let request = format!(
            "GET /api/v2/update?p={}&c={}&s={} HTTP/1.0\r\nHost: localhost\r\n\r\n",
            phext, coord, encoded
        );
        self.http_request(&request).is_some()
    }

    /// Raw HTTP request — no dependencies, minimal allocation
    fn http_request(&self, request: &str) -> Option<String> {
        let mut stream = TcpStream::connect_timeout(&self.addr, self.timeout).ok()?;
        stream.set_read_timeout(Some(self.timeout)).ok()?;
        stream.write_all(request.as_bytes()).ok()?;

        let mut response = Vec::new();
        stream.read_to_end(&mut response).ok()?;

        let text = String::from_utf8_lossy(&response);
        // Extract body after \r\n\r\n
        if let Some(body_start) = text.find("\r\n\r\n") {
            let body = text[body_start + 4..].trim().to_string();
            if body.is_empty() { None } else { Some(body) }
        } else {
            None
        }
    }
}

/// Minimal URL encoding (no external deps)
fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            b' ' => out.push_str("%20"),
            _ => {
                out.push('%');
                out.push(char::from_digit((b >> 4) as u32, 16).unwrap_or('0'));
                out.push(char::from_digit((b & 0xf) as u32, 16).unwrap_or('0'));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_parse_roundtrip() {
        let c = DroidCoord::parse("3.7.2/8.1.4/5.9.1").unwrap();
        assert_eq!(c.dims, [3, 7, 2, 8, 1, 4, 5, 9, 1]);
        assert_eq!(c.to_string(), "3.7.2/8.1.4/5.9.1");
    }

    #[test]
    fn test_message_roundtrip() {
        let msg = Message::Hello {
            coord: DroidCoord::parse("3.7.2/8.1.4/5.9.1").unwrap(),
            name: "Scout".into(),
            archetype: "phex".into(),
            scrolls: 847,
            power_state: "idle".into(),
        };
        let wire = msg.to_wire();
        let parsed = Message::from_wire(&wire).unwrap();
        match parsed {
            Message::Hello { coord, name, scrolls, .. } => {
                assert_eq!(name, "Scout");
                assert_eq!(scrolls, 847);
                assert_eq!(coord.to_string(), "3.7.2/8.1.4/5.9.1");
            }
            _ => panic!("wrong message type"),
        }
    }

    #[test]
    fn test_url_encode() {
        assert_eq!(url_encode("hello world"), "hello%20world");
        assert_eq!(url_encode("a+b=c"), "a%2bb%3dc");
    }
}
