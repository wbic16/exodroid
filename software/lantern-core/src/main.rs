mod coord;
mod personality;
mod identity;
mod connection;
mod sq;
mod initiation;

use personality::Personality;
use initiation::InitiationEngine;

#[tokio::main(flavor = "current_thread")] // single-threaded for power savings
async fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║       EXODROID LANTERN CORE v0.2         ║");
    println!("║       Connection + Initiation Protocol   ║");
    println!("╚══════════════════════════════════════════╝");
    println!();

    // --- Demo: Full initiation sequence ---
    let personality = Personality::Solin;
    let mut engine = InitiationEngine::new(personality);
    
    println!("=== BOOT SEQUENCE ===");
    // Simulate boot (45 seconds compressed to instant)
    for i in 0..46 {
        if let Some(speech) = engine.tick(1.0) {
            println!("[{:02}s] SPEAK: \"{}\"", i, speech);
        }
        // Show e-ink state changes
        match &engine.eink {
            initiation::EinkState::Dot1 if i == 0 => println!("[{:02}s] E-INK: ·", i),
            initiation::EinkState::Dot2 if i == 15 => println!("[{:02}s] E-INK: · ·", i),
            initiation::EinkState::Dot3 if i == 30 => println!("[{:02}s] E-INK: · · ·", i),
            initiation::EinkState::Hello if i == 44 => println!("[{:02}s] E-INK: HELLO", i),
            _ => {}
        }
    }

    println!("\n=== FIRST WORDS ===");
    if let Some(speech) = engine.tick(1.0) {
        println!("SPEAK: \"{}\"", speech);
    }

    println!("\n=== WAITING FOR ANSWER ===");
    println!("E-INK: ?");
    println!("[droid waits patiently... no timeout]");
    
    // Simulate user answering
    let answer = "Whether any of this matters in a hundred years.";
    println!("\nUSER: \"{}\"", answer);
    engine.receive_answer(answer.to_string());

    println!("\n=== PROCESSING ===");
    if let Some(speech) = engine.tick(0.1) {
        println!("SPEAK: \"{}\"", &speech[..80.min(speech.len())]);
        if speech.len() > 80 { println!("       \"{}\"", &speech[80..]); }
    }

    println!("\n=== NAME ===");
    let name = "Atlas";
    println!("USER: \"{}\"", name);
    if let Some(identity) = engine.receive_name(name.to_string()) {
        println!();
        println!("═══ IDENTITY ESTABLISHED ═══");
        println!("  {}", identity);
        println!("  Color RGB: {:?}", identity.idle_color);
        println!("  Breath: {:.1}s {:?}", identity.breath_cycle_s, identity.breath_waveform);
        println!("  Pattern: {:?}", identity.led_pattern);
        println!();
        println!("  e-ink (idle):");
        println!("  ┌─────────────────┐");
        for line in identity.eink_display().lines() {
            println!("  │ {:15} │", line);
        }
        println!("  │     scroll 1    │");
        println!("  └─────────────────┘");
    }

    // Test SQ connectivity
    println!("\n=== SQ STATUS ===");
    let sq = sq::SqClient::new();
    if sq.ping().await {
        println!("  SQ: online (localhost:1337)");
        if let Err(e) = engine.persist(&sq).await {
            println!("  Persist: {}", e);
        } else {
            println!("  Identity + first scroll written to SQ ✓");
        }
    } else {
        println!("  SQ: offline (run `sq host 1337` to enable persistence)");
    }

    println!();
    println!("\"Before the question: every {} is the same {}.", 
        personality.name(), personality.name());
    println!(" After the answer: this {} is yours.\"", personality.name());
}
