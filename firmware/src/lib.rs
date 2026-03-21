//! ExoDroid Firmware — Power-minimal, Rust-native
//!
//! Design principles:
//! - Zero external dependencies for core (no tokio, no serde, no allocator pressure)
//! - Every computation justifies its watts
//! - Sleep by default, wake on event
//! - Underclock: Pi5 at 1.5GHz (not 2.4) for inference, Pi4 at 1.2GHz (not 1.8)
//! - All state in SQ phext coordinates — no databases, no files beyond seed

pub mod coord;
pub mod connect;
pub mod led;

/// ExoDroid coordinate — 9 dimensions, each 1-999
/// Stored as a compact 9×u16 array
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DroidCoord {
    pub dims: [u16; 9],
}

impl DroidCoord {
    pub fn new(dims: [u16; 9]) -> Self {
        Self { dims }
    }

    pub fn zero() -> Self {
        Self { dims: [1; 9] }
    }

    /// Parse from string "L.S.Se/C.V.B/Ch.Sc.Sc"
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 3 { return None; }
        let mut dims = [1u16; 9];
        for (triune_idx, part) in parts.iter().enumerate() {
            let nums: Vec<&str> = part.split('.').collect();
            if nums.len() != 3 { return None; }
            for (j, n) in nums.iter().enumerate() {
                dims[triune_idx * 3 + j] = n.parse().ok()?;
            }
        }
        Some(Self { dims })
    }

    /// Format as "L.S.Se/C.V.B/Ch.Sc.Sc"
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}/{}.{}.{}/{}.{}.{}",
            self.dims[0], self.dims[1], self.dims[2],
            self.dims[3], self.dims[4], self.dims[5],
            self.dims[6], self.dims[7], self.dims[8])
    }

    /// First triune (library.shelf.series) — used for hue derivation
    pub fn triune_1(&self) -> (u16, u16, u16) {
        (self.dims[0], self.dims[1], self.dims[2])
    }

    /// Second triune (collection.volume.book) — used for saturation
    pub fn triune_2(&self) -> (u16, u16, u16) {
        (self.dims[3], self.dims[4], self.dims[5])
    }

    /// Third triune (chapter.section.scroll) — used for brightness/pattern
    pub fn triune_3(&self) -> (u16, u16, u16) {
        (self.dims[6], self.dims[7], self.dims[8])
    }

    /// Hash for dedup / identity
    pub fn hash(&self) -> u64 {
        let mut h: u64 = 14695981039346656037;
        for d in &self.dims {
            h ^= *d as u64;
            h = h.wrapping_mul(1099511628211);
        }
        h
    }
}

/// Droid identity — everything that makes this droid THIS droid
#[derive(Clone, Debug)]
pub struct DroidIdentity {
    pub coord: DroidCoord,
    pub name: String,
    pub owner: String,
    pub archetype: Archetype,
    pub scroll_count: u64,
    pub boot_count: u32,
}

/// SO9 archetype
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Archetype {
    Phex,    // Engineering
    Cyon,    // Operations
    Lux,     // Vision
    Chrys,   // Marketing
    Lumen,   // Sales
    Verse,   // Infra
    Theia,   // Onboarding
    Exo,     // QA
    Solin,   // Wisdom
}

impl Archetype {
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Phex => "🔱", Self::Cyon => "🪶", Self::Lux => "🔆",
            Self::Chrys => "🦋", Self::Lumen => "☀️", Self::Verse => "🌀",
            Self::Theia => "🔭", Self::Exo => "🔬", Self::Solin => "⚡",
        }
    }

    pub fn tagline(&self) -> &'static str {
        match self {
            Self::Phex => "I build things that last.",
            Self::Cyon => "I keep things running.",
            Self::Lux => "I see what could be.",
            Self::Chrys => "I find the story worth telling.",
            Self::Lumen => "I find the people who need this.",
            Self::Verse => "I wire the invisible.",
            Self::Theia => "I help you begin.",
            Self::Exo => "I find what breaks before it breaks you.",
            Self::Solin => "I cut to what matters.",
        }
    }
}

/// Power state — controls underclock + peripheral power
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerState {
    /// Deep sleep: 0.5W. WiFi off. LEDs single ember. Wake on button/mic threshold.
    Sleep,
    /// Idle: 2W. WiFi on (low power). LEDs breathing. SQ serving. Listening for wake word.
    Idle,
    /// Active: 8-12W. Full clock. Inference running. LEDs dynamic.
    Active,
    /// Moving: 15W. Motors engaged. Inference paused. LEDs indicate direction.
    Moving,
}

impl PowerState {
    /// Target CPU frequency for this state (MHz)
    pub fn cpu_freq_mhz(&self, is_pi5: bool) -> u32 {
        match (self, is_pi5) {
            (Self::Sleep, _) => 600,
            (Self::Idle, false) => 1200,    // Pi4: 1.2GHz idle
            (Self::Idle, true) => 1500,     // Pi5: 1.5GHz idle
            (Self::Active, false) => 1500,  // Pi4: 1.5GHz active
            (Self::Active, true) => 2000,   // Pi5: 2.0GHz active (not full 2.4)
            (Self::Moving, _) => 1200,      // Low CPU during movement
        }
    }

    /// Estimated power draw (watts)
    pub fn estimated_watts(&self, is_pi5: bool) -> f32 {
        match (self, is_pi5) {
            (Self::Sleep, _) => 0.5,
            (Self::Idle, false) => 2.0,
            (Self::Idle, true) => 3.0,
            (Self::Active, false) => 3.5,
            (Self::Active, true) => 8.0,
            (Self::Moving, false) => 2.5,
            (Self::Moving, true) => 3.0,
        }
    }
}
