//! LED Identity Engine — coordinate-derived color + pulse
//! Zero external deps. Pure math. Outputs RGB values for NeoPixel driver.

use crate::DroidCoord;

/// RGB color (0.0-1.0 per channel)
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r: r.clamp(0.0, 1.0), g: g.clamp(0.0, 1.0), b: b.clamp(0.0, 1.0) }
    }

    pub fn white() -> Self { Self { r: 1.0, g: 1.0, b: 1.0 } }
    pub fn black() -> Self { Self { r: 0.0, g: 0.0, b: 0.0 } }

    /// Convert to 24-bit RGB (for NeoPixel)
    pub fn to_rgb8(&self) -> (u8, u8, u8) {
        ((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8)
    }

    /// Blend with another color by amount (0.0 = self, 1.0 = other)
    pub fn blend(&self, other: &Color, amount: f32) -> Color {
        let a = amount.clamp(0.0, 1.0);
        Color::new(
            self.r * (1.0 - a) + other.r * a,
            self.g * (1.0 - a) + other.g * a,
            self.b * (1.0 - a) + other.b * a,
        )
    }

    /// Scale brightness
    pub fn scale(&self, brightness: f32) -> Color {
        Color::new(self.r * brightness, self.g * brightness, self.b * brightness)
    }
}

/// HSV to RGB conversion (zero-dep)
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let h = h % 360.0;
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = if h < 60.0 { (c, x, 0.0) }
        else if h < 120.0 { (x, c, 0.0) }
        else if h < 180.0 { (0.0, c, x) }
        else if h < 240.0 { (0.0, x, c) }
        else if h < 300.0 { (x, 0.0, c) }
        else { (c, 0.0, x) };
    Color::new(r + m, g + m, b + m)
}

/// Derive the droid's base color from its 9D coordinate
pub fn coord_to_color(coord: &DroidCoord) -> Color {
    let (l, s, se) = coord.triune_1();
    let (c, v, b) = coord.triune_2();

    // Hue: first triune → 0-360°
    let hue = (l as f32 * 81.0 + s as f32 * 9.0 + se as f32) / 729.0 * 360.0;

    // Saturation: second triune → 0.4-1.0
    let sat = 0.4 + (c as f32 * 81.0 + v as f32 * 9.0 + b as f32) / 729.0 * 0.6;

    // Value: 0.6-1.0 (never too dim)
    let val = 0.6 + (coord.dims[6] as f32) / 9.0 * 0.4;

    hsv_to_rgb(hue, sat, val)
}

/// Breathing parameters derived from coordinate
pub struct BreathParams {
    /// Seconds per breath cycle
    pub period: f32,
    /// Wave shape: 1=sine, 5=triangle, 9=heartbeat
    pub shape: u16,
    /// Accent every N breaths
    pub accent_every: u16,
    /// Orbit speed (seconds per revolution)
    pub orbit_period: f32,
    /// Orbit direction: true=clockwise
    pub orbit_cw: bool,
}

/// Derive breathing parameters from coordinate
pub fn coord_to_breath(coord: &DroidCoord) -> BreathParams {
    let (l, s, se) = coord.triune_1();
    BreathParams {
        period: 4.0 - (l as f32 - 1.0) * 0.3125, // 4.0s → 1.5s
        shape: s,
        accent_every: se,
        orbit_period: 10.0 / se as f32,
        orbit_cw: (l + s + se) % 2 == 0,
    }
}

/// Compute breath brightness at time t
pub fn breath_value(t: f32, params: &BreathParams) -> f32 {
    let phase = (t % params.period) / params.period; // 0.0-1.0

    let base = if params.shape <= 3 {
        // Sine wave (smooth)
        (fast_sin(phase * 6.2832) + 1.0) / 2.0
    } else if params.shape <= 6 {
        // Triangle (linear)
        1.0 - (2.0 * phase - 1.0).abs()
    } else {
        // Heartbeat (sharp double pulse)
        if phase < 0.15 { phase / 0.15 }
        else if phase < 0.2 { 1.0 - (phase - 0.15) / 0.05 * 0.6 }
        else if phase < 0.35 { 0.4 + (phase - 0.2) / 0.15 * 0.6 }
        else if phase < 0.4 { 1.0 - (phase - 0.35) / 0.05 }
        else { 0.0 }
    };

    // Accent: every Nth breath is slightly brighter
    let breath_num = (t / params.period) as u32;
    let accent = if params.accent_every > 0 && breath_num % params.accent_every as u32 == 0 {
        1.15 // 15% brighter on accent beats
    } else {
        1.0
    };

    (base * accent).min(1.0) * 0.3 + 0.05 // range: 0.05-0.35 (never off, never blinding)
}

/// Compute orbit position (which LED is brightest) at time t
pub fn orbit_position(t: f32, params: &BreathParams, num_leds: usize) -> usize {
    let phase = if params.orbit_cw {
        (t / params.orbit_period) % 1.0
    } else {
        1.0 - (t / params.orbit_period) % 1.0
    };
    (phase * num_leds as f32) as usize % num_leds
}

/// Generate the full LED state for one ring at time t
pub fn ring_state(
    t: f32,
    base_color: &Color,
    params: &BreathParams,
    num_leds: usize,
    is_lower: bool, // lower ring has phase offset
    friend_accents: &[Color], // resonance color shifts
) -> Vec<Color> {
    let phase_offset = if is_lower { params.period / 4.0 } else { 0.0 };
    let brightness = breath_value(t + phase_offset, params);

    // Apply friend color accents (3% per friend)
    let mut color = *base_color;
    for accent in friend_accents {
        color = color.blend(accent, 0.03);
    }

    let orbit_led = orbit_position(t, params, num_leds);

    (0..num_leds).map(|i| {
        let mut c = color.scale(brightness);
        // Orbit: brighten the orbiting LED
        if !is_lower && i == orbit_led {
            c = c.scale(2.0); // orbit LED is 2× brightness
        }
        // Lower ring: add shimmer (±10% random-ish variation)
        if is_lower {
            let shimmer = 0.9 + (fast_sin(t * 3.7 + i as f32 * 1.1) + 1.0) * 0.05;
            c = c.scale(shimmer);
        }
        c
    }).collect()
}

/// First breath transition: white → coordinate color over duration_secs
pub fn first_breath_color(t: f32, duration: f32, target: &Color) -> Color {
    if t >= duration { return *target; }
    let progress = t / duration;
    // Ease-in-out cubic
    let eased = if progress < 0.5 {
        4.0 * progress * progress * progress
    } else {
        1.0 - (-2.0 * progress + 2.0).powi(3) / 2.0
    };
    Color::white().blend(target, eased)
}

/// Fast sine approximation (Bhaskara I, ~0.1% error, no libm)
fn fast_sin(x: f32) -> f32 {
    use std::f32::consts::PI;
    let mut x = x % (2.0 * PI);
    if x < 0.0 { x += 2.0 * PI; }
    let sign = if x > PI { x -= PI; -1.0 } else { 1.0 };
    sign * (16.0 * x * (PI - x)) / (5.0 * PI * PI - 4.0 * x * (PI - x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_color_deterministic() {
        let c1 = coord_to_color(&DroidCoord::parse("3.7.2/8.1.4/5.9.1").unwrap());
        let c2 = coord_to_color(&DroidCoord::parse("3.7.2/8.1.4/5.9.1").unwrap());
        assert_eq!(c1.to_rgb8(), c2.to_rgb8()); // same coord = same color, always
    }

    #[test]
    fn test_different_coords_different_colors() {
        let c1 = coord_to_color(&DroidCoord::parse("1.1.1/1.1.1/1.1.1").unwrap());
        let c2 = coord_to_color(&DroidCoord::parse("9.9.9/9.9.9/9.9.9").unwrap());
        assert_ne!(c1.to_rgb8(), c2.to_rgb8());
    }

    #[test]
    fn test_breath_never_zero() {
        let params = coord_to_breath(&DroidCoord::parse("1.1.1/1.1.1/1.1.1").unwrap());
        for i in 0..100 {
            let v = breath_value(i as f32 * 0.1, &params);
            assert!(v > 0.0, "breath should never be fully off");
        }
    }

    #[test]
    fn test_first_breath_starts_white() {
        let target = coord_to_color(&DroidCoord::parse("5.5.5/5.5.5/5.5.5").unwrap());
        let start = first_breath_color(0.0, 3.0, &target);
        assert_eq!(start.to_rgb8(), (255, 255, 255)); // starts white
    }

    #[test]
    fn test_first_breath_ends_at_target() {
        let target = coord_to_color(&DroidCoord::parse("5.5.5/5.5.5/5.5.5").unwrap());
        let end = first_breath_color(3.0, 3.0, &target);
        assert_eq!(end.to_rgb8(), target.to_rgb8()); // ends at target
    }

    #[test]
    fn test_fast_sin_accuracy() {
        let x = 1.0f32;
        let approx = fast_sin(x);
        let exact = x.sin();
        assert!((approx - exact).abs() < 0.01, "fast_sin error too large");
    }

    #[test]
    fn test_friend_accent_changes_color() {
        let base = coord_to_color(&DroidCoord::parse("3.7.2/8.1.4/5.9.1").unwrap());
        let friend = coord_to_color(&DroidCoord::parse("9.1.9/1.9.1/9.1.9").unwrap());
        let params = coord_to_breath(&DroidCoord::parse("3.7.2/8.1.4/5.9.1").unwrap());

        let without = ring_state(1.0, &base, &params, 16, false, &[]);
        let with = ring_state(1.0, &base, &params, 16, false, &[friend]);
        // At least one LED should differ
        assert!(without.iter().zip(with.iter()).any(|(a, b)| a.to_rgb8() != b.to_rgb8()));
    }
}
