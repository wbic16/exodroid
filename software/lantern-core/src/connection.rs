/// Connection Protocol — how droids discover, bond, and sync
/// Minimal power: mDNS for discovery, SQ for state, NFC for seeds
use serde::{Serialize, Deserialize};
use crate::coord::Coord9D;
use crate::personality::Personality;

/// A discovered droid on the local network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredDroid {
    pub name: String,
    pub coord: Coord9D,
    pub personality: Personality,
    pub hostname: String,
    pub sq_port: u16,
    pub color_rgb: (u8, u8, u8),
    pub discovered_at: u64,
}

/// Bond between two droids — bilateral, consent-required
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bond {
    pub bond_id: String,
    pub partner_name: String,
    pub partner_coord: Coord9D,
    pub partner_personality: Personality,
    pub partner_color: (u8, u8, u8),
    pub bond_type: BondType,
    pub consented_at: u64,
    pub scroll_sharing: ScrollSharing,
    pub presence_effect: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BondType {
    Friend,   // topic-level scroll awareness, 5% color blend
    Family,   // full scroll access, breathing sync, 15% blend
    Partner,  // read/write scrolls, colors merge to third, 30% blend
    Mentor,   // one-directional scroll access, mentee brightens
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollSharing {
    None,       // unbonded
    Summaries,  // topic-level only (friend)
    ReadOnly,   // full read access (family, mentee→mentor)
    ReadWrite,  // bidirectional (partner)
}

impl BondType {
    /// Color blend factor when bonded droids are nearby
    pub fn color_blend(&self) -> f32 {
        match self {
            Self::Friend  => 0.05,
            Self::Family  => 0.15,
            Self::Partner => 0.30,
            Self::Mentor  => 0.0, // mentor doesn't change; mentee brightens
        }
    }

    /// Whether breathing should synchronize
    pub fn sync_breathing(&self) -> bool {
        matches!(self, Self::Family | Self::Partner)
    }

    pub fn scroll_sharing(&self) -> ScrollSharing {
        match self {
            Self::Friend  => ScrollSharing::Summaries,
            Self::Family  => ScrollSharing::ReadOnly,
            Self::Partner => ScrollSharing::ReadWrite,
            Self::Mentor  => ScrollSharing::ReadOnly,
        }
    }
}

/// Presence state — what we know about nearby droids
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceState {
    /// Currently discovered droids on the network
    pub nearby: Vec<DiscoveredDroid>,
    /// Active bonds
    pub bonds: Vec<Bond>,
    /// Ghost colors (from deceased bonded partners, 2% permanent blend)
    pub ghosts: Vec<(u8, u8, u8)>,
}

impl PresenceState {
    pub fn new() -> Self {
        Self {
            nearby: Vec::new(),
            bonds: Vec::new(),
            ghosts: Vec::new(),
        }
    }

    /// Check if a specific droid is currently nearby
    pub fn is_nearby(&self, coord: &Coord9D) -> bool {
        self.nearby.iter().any(|d| &d.coord == coord)
    }

    /// Get bond with a specific droid (if any)
    pub fn bond_with(&self, coord: &Coord9D) -> Option<&Bond> {
        self.bonds.iter().find(|b| &b.partner_coord == coord)
    }

    /// Calculate blended idle color given current presence
    /// Base color + bond blends + ghost traces
    pub fn blended_color(&self, base: (u8, u8, u8)) -> (u8, u8, u8) {
        let (mut r, mut g, mut b) = (base.0 as f32, base.1 as f32, base.2 as f32);
        let mut total_blend = 0.0f32;

        // Blend with nearby bonded droids
        for bond in &self.bonds {
            if self.is_nearby(&bond.partner_coord) {
                let factor = bond.bond_type.color_blend();
                let (pr, pg, pb) = bond.partner_color;
                r = r * (1.0 - factor) + pr as f32 * factor;
                g = g * (1.0 - factor) + pg as f32 * factor;
                b = b * (1.0 - factor) + pb as f32 * factor;
                total_blend += factor;
            }
        }

        // Ghost traces (2% each, always active)
        for ghost in &self.ghosts {
            let factor = 0.02;
            r = r * (1.0 - factor) + ghost.0 as f32 * factor;
            g = g * (1.0 - factor) + ghost.1 as f32 * factor;
            b = b * (1.0 - factor) + ghost.2 as f32 * factor;
        }

        (r.clamp(0.0, 255.0) as u8, g.clamp(0.0, 255.0) as u8, b.clamp(0.0, 255.0) as u8)
    }

    /// Calculate breathing sync target (weighted average of bonded droids' cycles)
    pub fn sync_breath_target(&self, own_cycle: f32) -> f32 {
        let syncing: Vec<f32> = self.bonds.iter()
            .filter(|b| b.bond_type.sync_breathing() && self.is_nearby(&b.partner_coord))
            .map(|_| own_cycle) // TODO: read partner's cycle from SQ
            .collect();

        if syncing.is_empty() {
            return own_cycle;
        }

        // Drift toward average (slow convergence, not instant snap)
        let avg = syncing.iter().sum::<f32>() / syncing.len() as f32;
        own_cycle * 0.95 + avg * 0.05 // 5% drift per tick toward group average
    }
}

/// mDNS service record for droid discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdnsRecord {
    pub service_type: String, // "_exodroid._tcp"
    pub hostname: String,
    pub port: u16,            // SQ port (1337)
    pub txt: MdnsTxt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdnsTxt {
    pub name: String,
    pub coord: String,
    pub personality: u8,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
}

impl MdnsRecord {
    /// Create mDNS advertisement for this droid
    pub fn advertise(
        hostname: &str,
        name: &str,
        coord: &Coord9D,
        personality: &Personality,
        color: (u8, u8, u8),
    ) -> Self {
        Self {
            service_type: "_exodroid._tcp".to_string(),
            hostname: hostname.to_string(),
            port: 1337,
            txt: MdnsTxt {
                name: name.to_string(),
                coord: coord.to_string(),
                personality: *personality as u8,
                color_r: color.0,
                color_g: color.1,
                color_b: color.2,
            },
        }
    }
}
