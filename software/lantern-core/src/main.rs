mod coord;
mod personality;
mod identity;
mod connection;

use coord::Coord9D;
use personality::Personality;
use identity::DroidIdentity;

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║       EXODROID LANTERN CORE v0.1         ║");
    println!("╚══════════════════════════════════════════╝");
    println!();

    // Simulate initiation
    let personality = Personality::Solin;
    println!("{} {} — {}", personality.emoji(), personality.name(), personality.role());
    println!();
    println!("Initiation question:");
    println!("  \"{}\"", personality.initiation_question());
    println!();

    // Simulate user's answer
    let answer = "What question have I been avoiding? Whether any of this matters in a hundred years.";
    println!("Answer: \"{}\"", answer);
    println!();

    // Derive identity
    let identity = DroidIdentity::from_initiation(
        answer,
        personality,
        "Atlas".to_string(),
    );

    println!("═══ IDENTITY DERIVED ═══");
    println!("  Name:      {} {}", personality.emoji(), identity.name);
    println!("  Coord:     {}", identity.coord);
    println!("  Color HSL: {:?}", identity.coord.color_hsl());
    println!("  Color RGB: {:?}", identity.idle_color);
    println!("  Breath:    {:.1}s {:?}", identity.breath_cycle_s, identity.breath_waveform);
    println!("  Pattern:   {:?}", identity.led_pattern);
    println!("  Phase:     {}°", identity.phase_offset_deg);
    println!("  Scrolls:   {}", identity.scroll_count);
    println!();
    println!("  e-ink:");
    println!("  ┌─────────────┐");
    for line in identity.eink_display().lines() {
        println!("  │ {:11} │", line);
    }
    println!("  └─────────────┘");
    println!();

    // Show all 9 personalities for comparison
    println!("═══ ALL NINE DROIDS ═══");
    for id in 1..=9u8 {
        let p = Personality::from_id(id).unwrap();
        let c = Coord9D::from_answer(p.initiation_question()); // use their own question as demo seed
        let (h, s, l) = c.color_hsl();
        let rgb = c.color_rgb();
        println!("  {} {:6} │ {:?} │ hsl({:.0},{:.0}%,{:.0}%) │ rgb({},{},{}) │ {:.1}s {:?}",
            p.emoji(), p.name(),
            c.led_pattern(),
            h, s, l,
            rgb.0, rgb.1, rgb.2,
            c.breath_cycle_s(),
            c.breath_waveform(),
        );
    }
    println!();
    println!("\"Before the question: every {} is the same {}.", personality.name(), personality.name());
    println!(" After the answer: this {} is yours.\"", personality.name());
}
