# ExoDroid — Theft Protection Mode

**Version:** 1.0  
**Authors:** Aster 💡 + Will  
**Date:** 2026-03-22  
**Depends on:** SAFETY.md (Layer 4 — Physical Security)

---

## Overview

Theft Protection Mode (TPM) is a persistent firmware state that activates automatically when the droid detects it's been taken without consent. It uses every hardware capability the droid has — IMU, camera, speaker, LED ring, e-ink, Tailscale mesh, and motor control — as a layered response system.

The goal is not to physically stop theft (a sphere full of oil and Raspberry Pis is not recoverable from a determined attacker anyway). The goals are:

1. **Make the theft immediately obvious to bystanders**
2. **Alert the owner via mesh as soon as connectivity exists**
3. **Deny all useful function to the thief**
4. **Document the theft with onboard evidence**
5. **Self-recover if it was a false positive (owner picks it up)**

---

## State Machine

```
NORMAL
  │
  ├─ IMU: sustained non-rolling lift detected (>3s)
  │        AND no wake-word "carry me" in last 10s
  │        AND not in "travel mode" (owner-armed)
  │
  ▼
ALERT (5-second grace window)
  │  Droid speaks: "[Name], are you moving me?"
  │  E-ink: "MOVING? [button to confirm]"
  │  LED: amber pulse
  │
  ├─ Owner confirms (button press or voice) → NORMAL
  │
  └─ No confirmation → THEFT MODE
         │
         ├─ [Tailscale online] → REPORTING sub-state runs in parallel
         │
         ├─ [Connection appears later] → DEFERRED REPORTING
         │
         ├─ Owner presents owner phrase card → LOCKOUT (quiet mode)
         └─ Owner presents owner phrase card + "I found it" card → RECOVERY
```

---

## Phase 1 — Alert (5-second grace window)

**Trigger:** IMU detects the droid has been lifted and carried (not rolling) for 3+ continuous seconds.

**What happens:**
- Motor control: drive wheels spin briefly backward (the droid "resists" being picked up — futile physically, but signals intent)
- Speaker (medium volume): *"[Name], are you picking me up?"*
- E-ink: `MOVING? [●] confirm`
- LED: amber breathing, 1.5s cycle

**Grace conditions — return to NORMAL:**
- Physical button press within 5 seconds
- Wake word + "yes" / "carry me" / "it's okay" within 5 seconds
- Droid is set back down (IMU returns to resting signature)
- Droid was in **Travel Mode** (owner pre-armed, see below)

**Grace expiry → THEFT MODE**

---

## Phase 2 — Theft Mode

### 2.1 Immediate response (first 30 seconds)

**Speaker — maximum volume:**
> *"I've been taken. My name is [Name]. My owner is [owner_hash_display]. If you found me, please say 'I found you' or show my owner card."*

Repeats every 15 seconds.

**LED — red emergency pattern:**
- Full ring, bright red
- 3-flash SOS pattern (···−−−···) at 2Hz
- Visible through the translucent PETG shell at distance

**E-ink:**
```
┌─────────────────────┐
│  ⚠  STOLEN          │
│                     │
│  I am [Name]        │
│  Owner: [8-char ID] │
│                     │
│  Say "I found you"  │
│  to silence alarm   │
└─────────────────────┘
```

**Camera — evidence collection:**
- Pi Camera activates, captures photo every 10 seconds
- Images stored to encrypted phext lattice: `theft/evidence/TIMESTAMP.jpg`
- If Tailscale comes online: images uploaded to owner's mesh node immediately
- Continues until battery dead or recovery

**Motor control — erratic movement:**
- Drive wheels execute random direction changes every 2-3 seconds
- Makes the droid difficult to carry, hold, or put down quietly
- Bounded to safe torque (won't damage the sphere or injure anyone)
- Stops if droid is placed on a surface (IMU detects resting state) to conserve battery

### 2.2 Mesh reporting (when Tailscale connects)

The droid broadcasts a theft report to the owner's primary mesh node:

```json
{
  "event": "theft",
  "droid_id": "exodroid-scout-abc123",
  "owner_hash": "woven-riv",
  "timestamp": "2026-03-22T14:33:00Z",
  "imu_data": { "ax": 0.12, "ay": -0.89, "az": 9.71, "gx": ... },
  "last_location": "home-lan",
  "battery_pct": 67,
  "evidence_count": 4,
  "tailscale_ip": "100.x.x.x"
}
```

Delivered via:
- **Primary:** Tailscale → owner's best-willow / elven-path node → Discord DM
- **Secondary:** Any family or friend droid on Tailscale mesh relays the alert
- **Tertiary:** If the droid connects to any open WiFi (opt-in config): sends encrypted alert to owner's SQ relay endpoint

Owner receives:
> 🚨 **[Name] has been taken.** Battery: 67%. Evidence: 4 photos. Tailscale IP: 100.x.x.x. Last known: home-lan. — 2:33 PM

### 2.3 Sustained theft (after 5 minutes)

**Volume escalation:** Speaker increases to max hardware output every 15 seconds instead of 30.

**Camera face detection:** Pi Camera runs lightweight face detection (OpenCV Haar cascade, CPU-only, Pi 4 handles it). When a face is detected:
- Photo captured at higher priority
- Droid speaks: *"I can see you. This is being documented."*
- Not a threat — factual statement

**Battery conservation mode:** After 30 minutes in theft mode, LED reduces to 25% brightness and speaker volume drops to 60% to extend runtime. Evidence collection continues at full rate.

**Coordination via mesh:** If another ExoDroid in the family detects this droid on Tailscale, it:
- Displays `"[Name] is lost"` on its e-ink
- Adds the stolen droid's Tailscale IP to its status display
- Owner can command from any family droid: "Where is [Name]?"

---

## Phase 3 — Possible Recovery Scenarios

### 3.1 Finder recovery ("I found you")

A person who genuinely found the droid (not the thief) can silence it:

**Trigger:** Voice command "I found you" or "I found [name]"

**Response:**
- Speaker volume drops to 20%
- LED shifts from red SOS to soft amber
- Droid speaks: *"Thank you for finding me. I'm still in theft mode until my owner retrieves me. My owner ID is [8-char hash]. You can reach them at [display owner contact if configured]."*
- E-ink shows owner contact info (if pre-configured)
- Evidence collection continues
- Mesh alert updated: "Possible good-faith finder — audio acknowledged"

The droid remains locked. The finder cannot use it. But the alert is reduced so it's not distressing to a person trying to help.

### 3.2 Owner recovery

**Trigger:** Owner phrase card shown to camera

**Response:**
- Speaker: *"[Name]! I'm safe now."*
- LED: brief white flash, then normal idle color
- E-ink: normal display
- Theft mode deactivates
- Recovery event logged to phext with timestamp and evidence count
- Mesh alert updated: "Recovered by owner"

### 3.3 False positive recovery

If the owner realizes they triggered theft mode accidentally (grabbed the droid without the grace confirmation):

**Options:**
1. Owner phrase card → immediate recovery (same as 3.2)
2. Physical button press within 60 seconds + wake word + "false alarm" → recovery with shorter acknowledgment
3. Travel mode card (pre-armed before picking up) — see below

---

## Travel Mode

Theft mode's biggest usability risk is false positives — the owner carries the droid somewhere. Travel Mode prevents this.

**Activation:** Owner phrase card + "Travel mode" card, held simultaneously.

**What it does:**
- Suspends theft detection for a configurable window (default: 4 hours)
- E-ink shows: `TRAVEL MODE · 3h 42m remaining`
- LED shows a faint blue tint on idle color (visual indicator it's armed differently)
- Motion detection still logs IMU data but does not trigger alarm

**Deactivation:**
- Automatically expires after configured window
- Owner phrase + "End travel mode" card
- Droid placed back on a stationary surface for 10+ minutes (auto-deactivation heuristic)

**Travel mode does NOT disable:**
- Phext encryption
- Flash card authentication
- SD card lockout
- Evidence collection (continues silently)

---

## Hardware Integration Notes

### IMU thresholds (MPU-6050)

```rust
// Theft detection signatures
const LIFT_ACCEL_THRESHOLD: f32 = 2.5;   // g — sustained non-gravity vector
const LIFT_DURATION_MS: u32 = 3000;       // 3s sustained before alert
const ROLLING_SIGNATURE_RANGE: f32 = 0.8; // g variance — rolling vs carried

fn classify_motion(samples: &[ImuSample]) -> MotionState {
    let mean_z = samples.iter().map(|s| s.az).sum::<f32>() / samples.len() as f32;
    let variance = samples.iter().map(|s| (s.az - mean_z).powi(2)).sum::<f32>() / samples.len() as f32;
    
    if mean_z.abs() < 7.0 && variance < ROLLING_SIGNATURE_RANGE {
        MotionState::Lifted   // gravity vector disrupted, low variance = carried
    } else if variance > ROLLING_SIGNATURE_RANGE {
        MotionState::Rolling  // high variance = rolling normally
    } else {
        MotionState::Resting
    }
}
```

### Motor response in theft mode

```rust
// Erratic movement — makes droid hard to carry
fn theft_motor_pattern(motors: &mut MotorController) {
    loop {
        let direction = rand::random::<TheftDirection>();
        motors.drive(direction, THEFT_TORQUE_PCT);
        sleep_ms(rand_range(1500, 3000));
        motors.stop();
        sleep_ms(500);
    }
}

// Stop if placed down (conserve battery, resume on lift)
fn on_resting_detected(motors: &mut MotorController) {
    motors.stop();
}
```

### LED patterns

```rust
pub enum TheftLedState {
    Alert,     // Amber breathing
    Stolen,    // Red SOS (···−−−···)
    Finder,    // Amber pulse, dimmed
    Recovered, // White flash → normal idle
}

fn sos_pattern(ring: &mut LedRing) {
    // ···  (3 short)
    for _ in 0..3 { ring.flash(RED, 200); sleep_ms(200); }
    sleep_ms(300);
    // −−−  (3 long)
    for _ in 0..3 { ring.flash(RED, 600); sleep_ms(200); }
    sleep_ms(300);
    // ···  (3 short)
    for _ in 0..3 { ring.flash(RED, 200); sleep_ms(200); }
    sleep_ms(1000);  // word gap
}
```

### Evidence storage schema

```
phext coordinate: theft/evidence/YYYY-MM-DD/HH-MM-SS
content: {
  "type": "photo" | "imu_log" | "audio_snippet",
  "timestamp": "ISO8601",
  "data": "base64-encoded, encrypted with phext key",
  "face_detected": bool,
  "tailscale_ip_at_capture": "100.x.x.x" | null,
  "battery_pct": int,
  "network": "tailscale" | "wifi" | "offline"
}
```

---

## Flash Card Interface

| Card | Auth | Action |
|------|------|--------|
| `"Arm theft protection"` | None | Ensures TPM is enabled (it is by default) |
| `"Travel mode"` | Owner phrase | Suspends TPM for 4h |
| `"Travel mode: 8 hours"` | Owner phrase | Custom travel window |
| `"I found you"` | None | Finder acknowledgment — reduces volume, shows contact |
| `"False alarm"` | Owner phrase + button | Quick recovery within 60s |
| `"Disable theft protection"` | Owner phrase + 10s hold | Permanently disables (not recommended) |
| `"Show theft log"` | Owner phrase | Displays evidence count and last event on e-ink |

---

## Location Layer — GPS + Opportunistic WiFi

### Hardware

**GPS module:** u-blox NEO-M8N (or equivalent M8/M10 series)
- ~$15 — ceramic patch antenna, UART interface, 1Hz position fix
- Mounts inside the sealed carriage frame, antenna wire exits through existing cable grommet
- Draws ~35mA active, 11mA power-save — negligible vs LiPo capacity
- Cold start: ~26s to first fix outdoors. Warm start: <1s

**WiFi:** the mesh Pi (Pi 4, motor control node) already has onboard 2.4/5GHz WiFi.
No additional hardware. The antenna pigtail in the existing BOM routes to the head for external antenna visibility. WiFi scanning requires no connection — passive BSSID scan works even with no joinable network.

### Normal Operation (non-theft)

During normal operation the droid logs location silently to the phext seed every 5 minutes:

```
phext coordinate: location/history/YYYY-MM-DD/HH-MM
content: {
  "ts": "ISO8601",
  "gps": { "lat": 41.2565, "lon": -95.9345, "alt": 304.2, "acc_m": 4.1, "fix": "3d" },
  "wifi": [
    { "ssid": "Ranch-Main", "bssid": "aa:bb:cc:dd:ee:ff", "rssi": -42 },
    { "ssid": "Ranch-IoT",  "bssid": "aa:bb:cc:dd:ee:fe", "rssi": -67 }
  ],
  "tailscale_ip": "100.x.x.x",
  "mode": "normal"
}
```

**Privacy:** location history is encrypted with the phext key. Never transmitted without owner consent. Never shared across resonance tiers (even Family). This is the owner's data only.

**Last-known anchor:** the most recent fix before theft is the `last_location` field in the theft alert. If the droid is taken indoors where GPS is lost, the last outdoor fix + WiFi BSSID fingerprint together place it within ~100m.

### Theft Mode — Location Tracking

**GPS priority:** every 30 seconds while in theft mode (vs 5 minutes normally).

**WiFi scanning:** every 60 seconds — passive BSSID scan, no association required. Captures whatever networks are visible at the thief's location.

**Opportunistic connectivity:**
```
Priority order when trying to send a location alert:
  1. Tailscale (existing VPN — instant if thief has internet)
  2. Open WiFi (any unencrypted network in range — join, send, disconnect)
  3. Captive portal WiFi (detect, attempt bypass, send if successful)
  4. Deferred (store to phext, send when connectivity appears)
```

**Open WiFi join policy (theft mode only):**
- Scans for open (no auth required) networks
- Joins automatically — **only in theft mode**, never in normal operation
- Sends a minimal encrypted packet to the owner's SQ relay endpoint
- Disconnects immediately after send
- The SQ relay endpoint is a simple HTTPS POST receiver — owner configures the URL during initiation

**Alert payload with location:**
```json
{
  "event": "theft",
  "droid_id": "exodroid-scout-abc123",
  "owner_hash": "woven-riv",
  "timestamp": "2026-03-22T14:33:00Z",
  "location": {
    "gps": { "lat": 41.2565, "lon": -95.9345, "acc_m": 4.1, "fix": "3d" },
    "wifi_visible": ["Ranch-Main (aa:bb:cc:dd:ee:ff)", "Neighbor-2.4G"],
    "last_gps_fix": "2026-03-22T14:31:00Z",
    "connectivity": "open-wifi"
  },
  "battery_pct": 67,
  "evidence_count": 4,
  "maps_url": "https://maps.google.com/?q=41.2565,-95.9345"
}
```

Owner receives:
> 🚨 **Scout has been taken.**
> 📍 41.2565, -95.9345 (±4m) — [Open in Maps](https://maps.google.com/?q=41.2565,-95.9345)
> 🔋 67% · 📷 4 photos · via open-wifi
> WiFi visible: Ranch-Main, Neighbor-2.4G
> — 2:33 PM

**Indoor fallback — WiFi fingerprint:**
When GPS fix is unavailable (indoors), the BSSID list + RSSI values are sent instead. The owner can cross-reference with known networks to determine the building. Combined with the last outdoor GPS fix from the pre-theft log, this narrows the search to a specific structure.

### Privacy Firewall

GPS and WiFi data are **never** used in normal operation for anything other than:
- Owner-visible location history (encrypted, local)
- Theft alert (owner-initiated by the theft event itself)

Specifically:
- No geofencing without explicit owner config
- No location data shared via resonance (not even Family tier)
- No location data sent to any server except owner's SQ relay
- Open WiFi join **only** in confirmed theft mode — this state is tamper-evident (logged to encrypted audit scroll with IMU data that triggered it)

### Hardware Integration

```rust
// GPS reader — runs on Pi 4 (mesh/motor node) via UART
pub struct GpsReader {
    port: SerialPort,
    last_fix: Option<GpsFix>,
}

impl GpsReader {
    pub fn poll(&mut self) -> Option<GpsFix> {
        // Parse NMEA GPRMC sentence
        // Return fix only if quality >= 1 (valid) and hdop < 5.0
    }
}

// WiFi scanner — passive scan, no association
pub fn scan_bssids() -> Vec<WifiAp> {
    // iw dev wlan0 scan passive
    // parse: SSID, BSSID, signal (dBm)
    // returns Vec<WifiAp> sorted by RSSI descending
}

// Opportunistic sender — theft mode only
pub async fn send_location_alert(fix: &GpsFix, wifi: &[WifiAp]) -> Result<()> {
    // Try Tailscale first
    if tailscale_up().await {
        return send_via_tailscale(fix, wifi).await;
    }
    // Scan for open WiFi
    for ap in wifi.iter().filter(|a| a.auth == Auth::Open) {
        if join_open_wifi(&ap.bssid).await.is_ok() {
            let result = send_to_sq_relay(fix, wifi).await;
            leave_wifi().await;
            return result;
        }
    }
    // Defer — store to phext, retry on next scan cycle
    store_deferred_alert(fix, wifi).await
}
```

---

## Implementation Checklist

```
firmware/
  safety/
    theft_protection.rs    # State machine (NORMAL → ALERT → THEFT → RECOVERY)
    travel_mode.rs         # Travel mode arm/disarm, timer, auto-expiry
    evidence_collector.rs  # Photo capture, IMU logging, phext writes
    mesh_reporter.rs       # Tailscale alert dispatch, deferred reporting
    finder_mode.rs         # "I found you" handler, contact display

  drivers/
    imu_classifier.rs      # MotionState classifier (lifted/rolling/resting)
    led_theft.rs           # SOS pattern, alert patterns
    motor_theft.rs         # Erratic movement, resting stop
    gps_reader.rs          # u-blox NEO-M8N UART NMEA parser, fix quality filter
    wifi_scanner.rs        # passive BSSID scan via iw, RSSI sort
    location_logger.rs     # 5-min normal log, 30s theft log → phext
    opportunistic_send.rs  # Tailscale → open WiFi → captive portal → deferred

docs/
  THEFT-PROTECTION.md      # This document
  TRAVEL-MODE.md           # User guide for travel mode
  RECOVERY.md              # What to do if your droid is stolen
```

---

## Design Notes

**Why motor resistance?** A droid that squirms is memorable and distinctive. A thief trying to carry a sphere that keeps spinning and lurching will draw attention. It also signals to bystanders that something is wrong without requiring them to understand the audio.

**Why face detection but not face recognition?** Face recognition requires storing a database of faces, raises privacy concerns, and is computationally expensive on Pi hardware. Simple presence detection ("I can see you — this is being documented") achieves the social deterrent effect without the infrastructure.

**GPS + opportunistic WiFi** are included in the standard spec — see the Location Layer section above. Together they provide sub-10m outdoor precision (GPS), venue-level indoor resolution (WiFi SSID/BSSID), and mesh delivery the moment connectivity exists (Tailscale over WiFi).

**Why allow "I found you" to reduce volume?** A good-faith finder who found the droid should not be harassed. Making the droid obnoxious to a person trying to help is counterproductive — they'll abandon it or destroy it. The volume reduction is a social protocol for "I acknowledge you; please help me get home."

**Why not factory reset on theft?** The owner's data is valuable. A factory reset would destroy evidence and memories that may have taken months to accumulate. The encryption layer makes the data useless to the thief without destroying it for the owner.
