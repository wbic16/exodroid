//! ExoDroid Preview — vectorized terminal visualization of the droid sphere
//! Shows: LED rings, thermochromic state, scroll count, coordinate, name
//! Usage: exodroid-preview --coord 3.7.2/8.1.4/5.9.1 --name Scout --scrolls 47

use exodroid_firmware::DroidCoord;
use exodroid_firmware::led::*;
use std::time::Instant;

const SPHERE_ART: &str = r#"
         ╭──────────╮
        ╱    ◉ {name}╲
       │   {eink_line1}  │
       │   {eink_line2}  │
        ╲  {coord_short} ╱
    ╭────┴──────────┴────╮
    │ {upper_ring}  │
    │                    │
    │    ┊ {scroll_ct} scrolls ┊    │
    │                    │
    │ {lower_ring}  │
    ╰────────────────────╯
"#;

fn render_ring(leds: &[Color], width: usize) -> String {
    let step = leds.len() / width.min(leds.len());
    let mut out = String::new();
    for i in 0..width {
        let led = &leds[(i * step) % leds.len()];
        let (r, g, b) = led.to_rgb8();
        out.push_str(&format!("\x1b[38;2;{};{};{}m●\x1b[0m", r, g, b));
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let coord_str = args.iter().position(|a| a == "--coord")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("3.7.2/8.1.4/5.9.1");
    let name = args.iter().position(|a| a == "--name")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("Scout");
    let scrolls: u64 = args.iter().position(|a| a == "--scrolls")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(3);
    let duration: f32 = args.iter().position(|a| a == "--duration")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(30.0);

    let coord = DroidCoord::parse(coord_str).expect("Invalid coordinate");
    let color = coord_to_color(&coord);
    let params = coord_to_breath(&coord);
    let (cr, cg, cb) = color.to_rgb8();

    println!("\x1b[2J\x1b[H"); // clear screen
    println!("  ExoDroid Preview — {} ({}) #{:02x}{:02x}{:02x}", name, coord.to_string(), cr, cg, cb);
    println!("  Ctrl-C to exit\n");

    let start = Instant::now();
    let num_leds = 16;

    loop {
        let t = start.elapsed().as_secs_f32();
        if t > duration { break; }

        // First 3 seconds: first breath (white → color)
        let display_color = if t < 3.0 {
            first_breath_color(t, 3.0, &color)
        } else {
            color
        };

        let upper = ring_state(t, &display_color, &params, num_leds, false, &[]);
        let lower = ring_state(t, &display_color, &params, num_leds, true, &[]);

        let upper_str = render_ring(&upper, 20);
        let lower_str = render_ring(&lower, 20);

        let breath = breath_value(t, &params);
        let (dr, dg, db) = display_color.scale(breath).to_rgb8();

        // Thermochromic state (simulated based on time as proxy for temp)
        let temp = 25.0 + breath * 15.0; // 25-40°C range based on "thinking"
        let thermo = if temp < 31.0 { "blue ■" }
            else if temp < 37.0 { "green ■" }
            else { "amber ■" };

        // Render sphere
        print!("\x1b[4;1H"); // move cursor to line 4
        println!("         ╭──────────╮");
        println!("        ╱    \x1b[38;2;{};{};{}m◉\x1b[0m {:6} ╲", dr, dg, db, &name[..name.len().min(6)]);
        println!("       │   {:10} │", coord_str);
        println!("       │   {:10} │", format!("{} scrolls", scrolls));
        println!("        ╲  thermo:{} ╱", thermo);
        println!("    ╭────┴──────────┴────╮");
        println!("    │ {}│", upper_str);
        println!("    │ \x1b[38;2;{};{};{}m    ████████████    \x1b[0m │", dr, dg, db);
        println!("    │ \x1b[38;2;{};{};{}m  ≈≈≈≈≈≈≈≈≈≈≈≈≈≈  \x1b[0m │", dr/2, dg/2, db/2);
        println!("    │ \x1b[38;2;{};{};{}m    ▓▓▓▓▓▓▓▓▓▓▓▓    \x1b[0m │", dr/3, dg/3, db/3);
        println!("    │ {}│", lower_str);
        println!("    ╰────────────────────╯");
        println!();
        println!("    t={:.1}s  breath={:.2}  temp={:.0}°C  orbit={}",
            t, breath, temp,
            if params.orbit_cw { "CW" } else { "CCW" });

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    println!("\n  Preview complete ({:.0}s)", duration);
}
