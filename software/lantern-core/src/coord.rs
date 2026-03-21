/// Phext 9D Coordinate — the droid's identity address
/// Derived from the user's initiation answer via SHA-256 → 9 values
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coord9D {
    /// 9 dimensions: library.shelf.series / collection.volume.book / chapter.section.scroll
    pub dims: [u16; 9],
}

impl Coord9D {
    pub fn new(dims: [u16; 9]) -> Self {
        Self { dims }
    }

    /// Derive a coordinate from the user's initiation answer
    /// SHA-256 of answer → split into 9 values, each mapped to 1..=999
    pub fn from_answer(answer: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(answer.as_bytes());
        let hash = hasher.finalize();

        let mut dims = [0u16; 9];
        for i in 0..9 {
            // Take 3 bytes per dimension (27 bytes of 32 used)
            let offset = i * 3;
            let val = ((hash[offset] as u32) << 16)
                | ((hash[offset + 1] as u32) << 8)
                | (hash[offset + 2] as u32);
            // Map to 1..=999
            dims[i] = (val % 999 + 1) as u16;
        }

        Self { dims }
    }

    /// Format as phext coordinate string: X.X.X/Y.Y.Y/Z.Z.Z
    pub fn to_string(&self) -> String {
        format!(
            "{}.{}.{}/{}.{}.{}/{}.{}.{}",
            self.dims[0], self.dims[1], self.dims[2],
            self.dims[3], self.dims[4], self.dims[5],
            self.dims[6], self.dims[7], self.dims[8],
        )
    }

    /// Triad 1 → HSL Hue (0..360)
    pub fn hue(&self) -> f32 {
        let val = (self.dims[0] as u32 * 137
            + self.dims[1] as u32 * 59
            + self.dims[2] as u32 * 23) % 360;
        val as f32
    }

    /// Triad 2 → Saturation (40..100)
    pub fn saturation(&self) -> f32 {
        let val = 40 + (self.dims[3] as u32 * 31
            + self.dims[4] as u32 * 17
            + self.dims[5] as u32 * 7) % 60;
        val as f32
    }

    /// Triad 3 → Lightness (30..70)
    pub fn lightness(&self) -> f32 {
        let val = 30 + (self.dims[6] as u32 * 43
            + self.dims[7] as u32 * 13
            + self.dims[8] as u32 * 3) % 40;
        val as f32
    }

    /// HSL color as (H, S%, L%)
    pub fn color_hsl(&self) -> (f32, f32, f32) {
        (self.hue(), self.saturation(), self.lightness())
    }

    /// Convert HSL to RGB (0..255)
    pub fn color_rgb(&self) -> (u8, u8, u8) {
        let (h, s, l) = self.color_hsl();
        hsl_to_rgb(h, s / 100.0, l / 100.0)
    }

    /// Breathing cycle length in seconds (3.0 – 6.0)
    pub fn breath_cycle_s(&self) -> f32 {
        3.0 + (self.dims[0] % 7) as f32 * 0.5
    }

    /// Breathing waveform type
    pub fn breath_waveform(&self) -> BreathWaveform {
        match self.dims[1] % 3 {
            0 => BreathWaveform::Sine,
            1 => BreathWaveform::Triangle,
            _ => BreathWaveform::Heartbeat,
        }
    }

    /// LED spatial pattern
    pub fn led_pattern(&self) -> LedPattern {
        match self.dims[4] % 9 {
            0 => LedPattern::Unified,
            1 => LedPattern::WaveUp,
            2 => LedPattern::WaveDown,
            3 => LedPattern::SpiralCW,
            4 => LedPattern::SpiralCCW,
            5 => LedPattern::Split,
            6 => LedPattern::Radiate,
            7 => LedPattern::Converge,
            _ => LedPattern::Firefly,
        }
    }

    /// Phase offset for multi-LED patterns (0..330 degrees)
    pub fn phase_offset_deg(&self) -> u16 {
        (self.dims[2] % 12) * 30
    }

    /// Owner key seed (for HMAC verification on droid seed)
    pub fn owner_key_seed(answer: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"exodroid-owner-key:");
        hasher.update(answer.as_bytes());
        let hash = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&hash);
        key
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreathWaveform {
    Sine,
    Triangle,
    Heartbeat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LedPattern {
    Unified,
    WaveUp,
    WaveDown,
    SpiralCW,
    SpiralCCW,
    Split,
    Radiate,
    Converge,
    Firefly,
}

/// Convert HSL (h: 0-360, s: 0-1, l: 0-1) to RGB (0-255)
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    if s == 0.0 {
        let v = (l * 255.0) as u8;
        return (v, v, v);
    }

    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let h_norm = h / 360.0;

    let r = hue_to_rgb(p, q, h_norm + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h_norm);
    let b = hue_to_rgb(p, q, h_norm - 1.0 / 3.0);

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 { t += 1.0; }
    if t > 1.0 { t -= 1.0; }
    if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; }
    if t < 1.0 / 2.0 { return q; }
    if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
    p
}

impl std::fmt::Display for Coord9D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic() {
        let c1 = Coord9D::from_answer("the stars are beautiful tonight");
        let c2 = Coord9D::from_answer("the stars are beautiful tonight");
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_different_answers_different_coords() {
        let c1 = Coord9D::from_answer("I want to build a bridge");
        let c2 = Coord9D::from_answer("I want to build a wall");
        assert_ne!(c1, c2);
    }

    #[test]
    fn test_range() {
        let c = Coord9D::from_answer("anything at all");
        for d in c.dims {
            assert!(d >= 1 && d <= 999, "dim {} out of range", d);
        }
    }

    #[test]
    fn test_color_range() {
        let c = Coord9D::from_answer("test color derivation");
        let (h, s, l) = c.color_hsl();
        assert!(h >= 0.0 && h < 360.0);
        assert!(s >= 40.0 && s <= 100.0);
        assert!(l >= 30.0 && l <= 70.0);
    }

    #[test]
    fn test_breath_range() {
        let c = Coord9D::from_answer("test breathing");
        let cycle = c.breath_cycle_s();
        assert!(cycle >= 3.0 && cycle <= 6.0);
    }

    #[test]
    fn test_display() {
        let c = Coord9D::new([3, 1, 4, 1, 5, 9, 2, 6, 5]);
        assert_eq!(c.to_string(), "3.1.4/1.5.9/2.6.5");
    }
}
