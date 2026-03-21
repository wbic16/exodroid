/// Droid Identity — the complete soul, derived from initiation
use serde::{Serialize, Deserialize};
use crate::coord::Coord9D;
use crate::personality::Personality;

/// Complete droid identity — everything derived from the initiation answer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DroidIdentity {
    /// User-chosen name (spoken during initiation)
    pub name: String,
    /// 9D coordinate (derived from initiation answer)
    pub coord: Coord9D,
    /// Personality archetype (factory-set, 1 of 9)
    pub personality: Personality,
    /// Owner key (HMAC seed for droid seed verification)
    #[serde(skip_serializing)]
    pub owner_key: [u8; 32],
    /// Scroll count (increments with each conversation stored)
    pub scroll_count: u64,
    /// Timestamp of initiation
    pub initiated_at: String,
    /// LED color after coordinate blending (RGB)
    pub idle_color: (u8, u8, u8),
    /// Breathing cycle (seconds)
    pub breath_cycle_s: f32,
    /// Breathing waveform
    pub breath_waveform: crate::coord::BreathWaveform,
    /// LED spatial pattern
    pub led_pattern: crate::coord::LedPattern,
    /// Phase offset (degrees)
    pub phase_offset_deg: u16,
}

impl DroidIdentity {
    /// Create identity from initiation answer + personality + chosen name
    pub fn from_initiation(
        answer: &str,
        personality: Personality,
        name: String,
    ) -> Self {
        let coord = Coord9D::from_answer(answer);
        let owner_key = Coord9D::owner_key_seed(answer);

        // Blend personality base color with coordinate accent (30%)
        let (ph, ps, pl) = personality.base_color_hsl();
        let (ch, cs, cl) = coord.color_hsl();
        let blend = 0.3;
        let blended_h = ph * (1.0 - blend) + ch * blend;
        let blended_s = ps * (1.0 - blend) + cs * blend;
        let blended_l = pl * (1.0 - blend) + cl * blend;

        let idle_color = crate::coord::hsl_to_rgb(blended_h, blended_s / 100.0, blended_l / 100.0);

        let now = chrono_lite_now();

        Self {
            name,
            coord,
            personality,
            owner_key,
            scroll_count: 1, // first scroll is the initiation answer itself
            initiated_at: now,
            idle_color,
            breath_cycle_s: coord.breath_cycle_s(),
            breath_waveform: coord.breath_waveform(),
            led_pattern: coord.led_pattern(),
            phase_offset_deg: coord.phase_offset_deg(),
        }
    }

    /// Verify an owner claim (re-answer the initiation question)
    pub fn verify_owner(&self, answer: &str) -> bool {
        let claimed_key = Coord9D::owner_key_seed(answer);
        self.owner_key == claimed_key
    }

    /// Increment scroll count
    pub fn record_scroll(&mut self) {
        self.scroll_count += 1;
    }

    /// Serialize to JSON for SQ storage
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Summary line for e-ink display
    pub fn eink_display(&self) -> String {
        format!("{} {}\n{}", self.personality.emoji(), self.name, self.coord)
    }
}

/// Simple timestamp without chrono dependency (power-minimal)
fn chrono_lite_now() -> String {
    // Use std::time for minimal deps
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Basic ISO-ish format from epoch seconds
    format!("epoch:{}", secs)
}

impl std::fmt::Display for DroidIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} @ {} (scrolls: {})",
            self.personality, self.name, self.coord, self.scroll_count)
    }
}
