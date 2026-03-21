//! Coordinate utilities — parsing, formatting, navigation
//! Re-exports from lib.rs DroidCoord with additional helpers

use crate::DroidCoord;

/// Generate a random-ish coordinate from a seed (deterministic, no RNG dependency)
pub fn coord_from_seed(seed: u64) -> DroidCoord {
    let mut h = seed;
    let mut dims = [1u16; 9];
    for i in 0..9 {
        h = h.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        dims[i] = (h % 9 + 1) as u16; // 1-9 per dimension
    }
    DroidCoord::new(dims)
}

/// Coordinate distance (Manhattan distance across all 9 dimensions, normalized)
pub fn coord_distance(a: &DroidCoord, b: &DroidCoord) -> f32 {
    let sum: u32 = a.dims.iter().zip(b.dims.iter())
        .map(|(x, y)| (*x as i32 - *y as i32).unsigned_abs())
        .sum();
    sum as f32 / (9.0 * 8.0) // normalize to 0.0-1.0 (max distance = 9 dims × 8 steps)
}

/// Check if two coordinates are "nearby" (distance < threshold)
pub fn coords_nearby(a: &DroidCoord, b: &DroidCoord, threshold: f32) -> bool {
    coord_distance(a, b) < threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_from_seed_deterministic() {
        let a = coord_from_seed(42);
        let b = coord_from_seed(42);
        assert_eq!(a.dims, b.dims);
    }

    #[test]
    fn test_coord_from_seed_different() {
        let a = coord_from_seed(42);
        let b = coord_from_seed(43);
        assert_ne!(a.dims, b.dims);
    }

    #[test]
    fn test_distance_self_zero() {
        let c = DroidCoord::parse("5.5.5/5.5.5/5.5.5").unwrap();
        assert_eq!(coord_distance(&c, &c), 0.0);
    }

    #[test]
    fn test_distance_max() {
        let a = DroidCoord::parse("1.1.1/1.1.1/1.1.1").unwrap();
        let b = DroidCoord::parse("9.9.9/9.9.9/9.9.9").unwrap();
        let d = coord_distance(&a, &b);
        assert!((d - 1.0).abs() < 0.01, "max distance should be ~1.0");
    }
}
