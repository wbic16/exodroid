//! ExoDroid Connect — peer discovery + SQ sync daemon
//! Usage: exodroid-connect --coord 3.7.2/8.1.4/5.9.1 --name Scout --archetype phex

use exodroid_firmware::{DroidCoord, PowerState};
use exodroid_firmware::connect::{ConnectConfig, SqClient, Message};
use std::time::{Duration, Instant};
use std::thread;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let coord_str = args.iter().position(|a| a == "--coord")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("1.1.1/1.1.1/1.1.1");
    let name = args.iter().position(|a| a == "--name")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("ExoDroid");
    let archetype = args.iter().position(|a| a == "--archetype")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("theia");
    let sq_host = args.iter().position(|a| a == "--sq")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("127.0.0.1");

    let coord = DroidCoord::parse(coord_str).expect("Invalid coordinate");
    let config = ConnectConfig::for_power_state(PowerState::Idle);
    let sq = SqClient::new(sq_host, config.sq_port, config.connect_timeout_ms);

    println!("ExoDroid Connect v0.1.0");
    println!("  Coord:     {}", coord.to_string());
    println!("  Name:      {}", name);
    println!("  Archetype: {}", archetype);
    println!("  SQ:        {}:{}", sq_host, config.sq_port);
    println!("  Poll:      {}s", config.poll_interval_secs);
    println!();

    // Write HELLO to local SQ
    let hello = Message::Hello {
        coord,
        name: name.to_string(),
        archetype: archetype.to_string(),
        scrolls: 0,
        power_state: "idle".to_string(),
    };

    let connect_coord = format!("connect/{}/1.1.1/1.1.1", coord.dims[0]);
    if sq.update("droid-status", &connect_coord, &hello.to_wire()) {
        println!("✅ HELLO written to SQ");
    } else {
        println!("⚠️  SQ not reachable at {}:{}", sq_host, config.sq_port);
    }

    // Main loop — poll for peers, respond to messages
    let mut last_poll = Instant::now();
    println!("Listening...");

    loop {
        if last_poll.elapsed() >= Duration::from_secs(config.poll_interval_secs as u64) {
            // Read incoming messages from SQ
            if let Some(msg) = sq.select("droid-inbox", &format!("{}/1.1.1/1.1.1", coord.dims[0])) {
                if let Some(parsed) = Message::from_wire(&msg) {
                    match parsed {
                        Message::Ping { coord: from } => {
                            println!("  PING from {}", from.to_string());
                            let pong = Message::Pong {
                                coord,
                                scrolls: 0,
                                power_state: "idle".to_string(),
                            };
                            let _ = sq.update("droid-inbox",
                                &format!("{}/1.1.1/1.1.1", from.dims[0]),
                                &pong.to_wire());
                        }
                        Message::Graft { from_coord, content, .. } => {
                            println!("  GRAFT from {}: {}", from_coord.to_string(),
                                &content[..content.len().min(50)]);
                        }
                        Message::Resonate { from_coord, tier } => {
                            println!("  RESONATE from {} tier={}", from_coord.to_string(), tier);
                        }
                        _ => {}
                    }
                }
            }
            last_poll = Instant::now();
        }

        thread::sleep(Duration::from_millis(100));
    }
}
