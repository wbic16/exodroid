//! LED Identity Preview — renders the coordinate-derived LED pattern to terminal
//! Usage: led-identity --coord 3.7.2/8.1.4/5.9.1 [--duration 10]

use exodroid_firmware::DroidCoord;
use exodroid_firmware::led::*;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let coord_str = args.iter().position(|a| a == "--coord")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("3.7.2/8.1.4/5.9.1");
    let duration: f32 = args.iter().position(|a| a == "--duration")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(10.0);

    let coord = DroidCoord::parse(coord_str).expect("Invalid coordinate");
    let color = coord_to_color(&coord);
    let params = coord_to_breath(&coord);
    let (r, g, b) = color.to_rgb8();

    println!("LED Identity for {}", coord.to_string());
    println!("  Base color: RGB({}, {}, {})  #{:02x}{:02x}{:02x}", r, g, b, r, g, b);
    println!("  Breath period: {:.2}s", params.period);
    println!("  Wave shape: {} ({})", params.shape,
        if params.shape <= 3 { "sine" } else if params.shape <= 6 { "triangle" } else { "heartbeat" });
    println!("  Accent every: {} breaths", params.accent_every);
    println!("  Orbit: {:.1}s/rev {}", params.orbit_period,
        if params.orbit_cw { "CW" } else { "CCW" });
    println!();

    // Animate in terminal
    let start = Instant::now();
    let num_leds = 16;

    // First breath: white → color
    println!("=== First Breath (3s) ===");
    for frame in 0..30 {
        let t = frame as f32 * 0.1;
        let c = first_breath_color(t, 3.0, &color);
        let (r, g, b) = c.to_rgb8();
        print!("\r  t={:.1}s  ", t);
        print!("\x1b[38;2;{};{};{}m████████████████\x1b[0m", r, g, b);
        std::io::Write::flush(&mut std::io::stdout()).ok();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!();
    println!();

    // Steady state breathing
    println!("=== Breathing Pattern ({}s) ===", duration);
    let anim_start = Instant::now();
    while anim_start.elapsed().as_secs_f32() < duration {
        let t = anim_start.elapsed().as_secs_f32();
        let upper = ring_state(t, &color, &params, num_leds, false, &[]);
        let lower = ring_state(t, &color, &params, num_leds, true, &[]);

        print!("\r  t={:5.1}s  Upper: ", t);
        for c in &upper {
            let (r, g, b) = c.to_rgb8();
            print!("\x1b[38;2;{};{};{}m█\x1b[0m", r, g, b);
        }
        print!("  Lower: ");
        for c in &lower {
            let (r, g, b) = c.to_rgb8();
            print!("\x1b[38;2;{};{};{}m█\x1b[0m", r, g, b);
        }

        std::io::Write::flush(&mut std::io::stdout()).ok();
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    println!();
    println!();
    println!("Total time: {:.1}s", start.elapsed().as_secs_f32());
}
