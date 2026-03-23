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

docs/
  THEFT-PROTECTION.md      # This document
  TRAVEL-MODE.md           # User guide for travel mode
  RECOVERY.md              # What to do if your droid is stolen
```

---

## Design Notes

**Why motor resistance?** A droid that squirms is memorable and distinctive. A thief trying to carry a sphere that keeps spinning and lurching will draw attention. It also signals to bystanders that something is wrong without requiring them to understand the audio.

**Why face detection but not face recognition?** Face recognition requires storing a database of faces, raises privacy concerns, and is computationally expensive on Pi hardware. Simple presence detection ("I can see you — this is being documented") achieves the social deterrent effect without the infrastructure.

**Why not GPS?** No GPS in the base spec. The Tailscale IP gives a rough network-topology location, and the camera provides physical evidence. GPS would require an additional module, antenna, and power budget. Worth adding as a v2 option for the SENTINEL archetype.

**Why allow "I found you" to reduce volume?** A good-faith finder who found the droid should not be harassed. Making the droid obnoxious to a person trying to help is counterproductive — they'll abandon it or destroy it. The volume reduction is a social protocol for "I acknowledge you; please help me get home."

**Why not factory reset on theft?** The owner's data is valuable. A factory reset would destroy evidence and memories that may have taken months to accumulate. The encryption layer makes the data useless to the thief without destroying it for the owner.
