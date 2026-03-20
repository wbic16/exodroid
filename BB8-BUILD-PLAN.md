# BB-8 Exocortex Droid — Modern Build Plan
**Version:** 1.0  
**Architect:** Orin 🖖 + Will  
**Date:** 2026-03-20  
**Reference:** Instructables 3D Printed BB-8 (hamster-drive design)  
**Focus:** Thermal management, inference-while-moving, fully offline

---

## Design Philosophy

This is not a toy. This is a personal Exocortex in BB-8 form factor — running local inference on 3 Raspberry Pis submerged in mineral oil inside a 3D-printed sphere. No internet required. No cloud. The droid thinks, moves, speaks, listens, and remembers — on its own.

**Key constraints:**
1. All compute on-device (no WiFi/internet dependency)
2. Thermal management for sustained inference while moving
3. Oil slosh management inside a rolling sphere
4. 3+ hour battery life at booth duty cycle
5. Printable on a consumer 3D printer (Ender 3 / Prusa i3 build volume)

---

## Architecture Overview

```
                ┌──── MAGNETIC HEAD ────┐
                │  Speaker + Mic        │
                │  Pi Camera Module 3   │
                │  e-ink coordinate     │
                │  Magnet array (N52)   │
                │  IMU (head tracking)  │
                └───────┬───────────────┘
                        │ (magnetic coupling)
    ╔═══════════════════╧════════════════════╗
    ║           SPHERE (3D printed)          ║
    ║                                        ║
    ║    ┌────────────────────────────┐      ║
    ║    │     SEALED OIL CHAMBER    │      ║
    ║    │                           │      ║
    ║    │  ┌──────┐ Pi 5 (brain)   │      ║
    ║    │  │ Pi 5 │ Ollama 7B 4-bit│      ║
    ║    │  └──────┘                │      ║
    ║    │  ┌──────┐ Pi 4 (store)   │      ║
    ║    │  │ Pi 4 │ SQ + Qdrant    │      ║
    ║    │  └──────┘                │      ║
    ║    │  ┌──────┐ Pi 4 (mesh)    │      ║
    ║    │  │ Pi 4 │ Tailscale+Kiwix│      ║
    ║    │  └──────┘                │      ║
    ║    │  [baffles for slosh mgmt]│      ║
    ║    │  [aquarium pump]         │      ║
    ║    └────────────────────────────┘      ║
    ║                                        ║
    ║    ┌────────────────────────────┐      ║
    ║    │     DRIVE CARRIAGE        │      ║
    ║    │  2× 12V DC gear motors   │      ║
    ║    │  Motor controller (L298N) │      ║
    ║    │  IMU (MPU-6050)          │      ║
    ║    │  Flywheel (yaw control)  │      ║
    ║    │  4S LiPo 5000mAh        │      ║
    ║    │  Power distribution      │      ║
    ║    └────────────────────────────┘      ║
    ║                                        ║
    ║    [counterweight (bottom)]            ║
    ╚════════════════════════════════════════╝
```

---

## Sphere Design

### Dimensions
- **Outer diameter:** 300mm (12 inches) — large enough for all components, small enough to 3D print in hemispheres
- **Shell thickness:** 3mm (PETG for impact resistance)
- **Print method:** Split into upper hemisphere + lower hemisphere + equatorial ring
- **Join method:** M3 bolts at equator ring (16 points) + gasket for oil-chamber seal

### Printing
- **Material:** PETG (not PLA — PLA warps at high temps, PETG handles 80°C)
- **Infill:** 20% gyroid (strength with minimal weight)
- **Layer height:** 0.2mm (quality vs speed balance)
- **Print time per hemisphere:** ~18-24h on Ender 3 (300mm is close to max build volume)
- **If build plate too small:** split each hemisphere into 4 segments, join with epoxy + fiberglass tape

### Surface Finish
- Sand to 400 grit → XTC-3D epoxy coating → primer → BB-8 orange/white paint scheme
- Alternative: vinyl wrap (faster, removable)

---

## Oil Chamber (sealed, baffled)

### Construction
- **Container:** 3D-printed rectangular box, sealed with silicone gasket
- **Internal dimensions:** 130 × 100 × 120mm (fits 3-Pi stack)
- **Volume:** ~1.5L (includes baffle volume)
- **Material:** PETG, 2mm walls, filleted corners
- **Seal:** silicone O-ring at lid, M3 bolts with nylon washers

### Baffle System (anti-slosh)
The sphere rolls. Oil sloshes. Slosh shifts the center of gravity unpredictably, destabilizing the carriage.

**Solution: perforated baffle plates**
```
┌─────────────────────┐
│  ┌─┐ ┌─┐ ┌─┐ ┌─┐  │  ← baffle plate (perforated)
│  │ │ │ │ │ │ │ │  │     oil flows through holes slowly
│  └─┘ └─┘ └─┘ └─┘  │     slosh energy absorbed
│  ══════════════════ │  ← Pi board (component acts as baffle too)
│  ┌─┐ ┌─┐ ┌─┐ ┌─┐  │  ← second baffle
│  │ │ │ │ │ │ │ │  │
│  └─┘ └─┘ └─┘ └─┘  │
│  ══════════════════ │  ← Pi board
│  ┌─┐ ┌─┐ ┌─┐ ┌─┐  │  ← third baffle
│  │ │ │ │ │ │ │ │  │
│  └─┘ └─┘ └─┘ └─┘  │
│  ══════════════════ │  ← Pi board
└─────────────────────┘
```

- 3mm PETG baffle plates between each Pi
- 6mm diameter holes, 40% open area (enough flow for thermal convection, enough resistance to kill slosh)
- The Pi PCBs themselves act as partial baffles
- Total baffle count: 5 (2 between Pis + 1 top + 1 bottom + Pi PCBs)

### Oil Filling
- Fill through a port with check valve at top of chamber
- Leave 5% air gap for thermal expansion
- Food-grade mineral oil (USP grade from pharmacy)

### Thermal Performance (in-sphere)
```
Pi 5 active (12W) + 2× Pi 4 idle (5.4W) = 17.4W
Oil thermal resistance: ~1.5°C/W
Sphere interior ambient (Oshkosh, shaded by head): ~40°C

Die temperature: 40 + (17.4 × 1.5) = 66°C
Thermal headroom: 19°C below throttle ← safe
```

Aquarium pump (USB-powered, inside oil chamber):
- Forces oil circulation → reduces theta to ~1.0°C/W
- Die temp drops to: 40 + (17.4 × 1.0) = 57°C
- **23°C headroom** with pump running

---

## Drive Carriage (hamster-drive mechanism)

### Principle
The carriage sits at the bottom of the sphere, weighted so it always stays down (pendulum). Two wheels press against the inside of the sphere, driving it to roll. The sphere rotates around the carriage.

```
        ┌──── sphere shell ────┐
        │                      │
        │    ○ flywheel        │  ← yaw control
        │                      │
        │  ┌──────────────┐   │
        │  │   OIL CHAMBER│   │  ← mounted to carriage frame
        │  │   (Pi stack) │   │
        │  └──────────────┘   │
        │                      │
        │  ┌──┐  motor  ┌──┐  │
        │  │▓▓│←──────→│▓▓│  │  ← drive wheels (silicone coated)
        │  └──┘         └──┘  │
        │  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  │  ← counterweight (brass bar)
        └──────────────────────┘
```

### Components

| Component | Model | Specs | Price |
|---|---|---|---|
| Drive motors (×2) | JGB37-520 12V DC gear motor | 30 RPM, 10kg·cm torque | $12 ea |
| Motor driver | L298N H-bridge | Dual channel, 12V 2A per channel | $5 |
| Drive wheels (×2) | 65mm silicone wheel | High-grip, press against sphere interior | $8 pair |
| Flywheel motor | NEMA 17 stepper | Yaw torque via angular momentum | $12 |
| Flywheel disc | 3D printed, brass-weighted | 200g, 80mm diameter | $5 |
| Counterweight | Brass bar | 500g, bottom of carriage | $8 |
| IMU | MPU-6050 (GY-521) | 6-axis gyro+accelerometer | $4 |
| Casters (×2) | 15mm ball casters | Support carriage, low friction | $3 pair |

### Carriage Frame
- 3D printed PETG
- U-shaped cradle: motors at bottom, oil chamber mounted above
- Counterweight (brass) at very bottom — ensures pendulum stability
- Total carriage weight with oil chamber: ~2.2 kg
- Center of gravity: 40mm below sphere center (stability margin)

### Movement Control

**Forward/backward:** both drive motors same direction → sphere rolls  
**Turning:** differential drive → sphere turns  
**Yaw (heading change without translation):** flywheel spins → reaction torque rotates carriage relative to sphere  
**Self-righting:** counterweight ensures carriage always returns to bottom-heavy orientation

**Control loop (runs on Pi 4 mesh node):**
```
IMU reads → orientation estimate (complementary filter)
  → PID controller adjusts motor speeds
  → Flywheel adjusts heading
  → Update rate: 100Hz (10ms loop)
  → Pi 4 handles this easily (<5% CPU)
```

The Pi 5 brain is NOT involved in movement control. It only does inference. The Pi 4 (mesh node) runs the motor control loop — keeps inference latency isolated from motor jitter.

---

## Head Assembly (magnetic)

### Principle
The head rides on top of the sphere, held by magnets. Internal magnets in the carriage attract external magnets in the head through the sphere shell. As the carriage stays upright, the head stays on top.

### Components

| Component | Model | Specs | Price |
|---|---|---|---|
| Internal magnets (×4) | N52 neodymium disc | 20mm × 5mm, 12kg pull each | $10 |
| External magnets (×4) | N52 neodymium disc | 15mm × 5mm (in head base) | $8 |
| Head servo (×2) | SG90 micro servo | Pan + tilt for head expressiveness | $4 pair |
| Speaker | 3W 40mm driver | Voice output, mounted in head | $5 |
| Microphone | I2S MEMS (INMP441) | Always-on listening, directional | $4 |
| Camera | Pi Camera Module 3 | For instruction cards + visual | $25 |
| e-ink display | Waveshare 1.54" | Shows coordinate, personality emoji | $15 |
| Head IMU | BNO055 | 9-axis, tracks head-vs-body orientation | $12 |

### Head Tracking
Head servo tracks the loudest audio source (beamforming with MEMS mic) — the head "looks at" whoever is speaking. The head IMU provides feedback to maintain gaze direction even as the sphere rolls underneath.

### Head Design
- 3D printed, 100mm hemisphere
- BB-8 style "eye" lens over camera
- e-ink display visible through a small window
- Speaker fires downward (bounces off sphere for omnidirectional audio)
- Weight: ~200g total (magnets are heaviest component)

---

## Power System

### Battery
**Primary:** 4S LiPo 5000mAh (74Wh, 430g)
- Mounted low in carriage (contributes to counterweight)
- XT60 quick-disconnect for hot swap
- **Runtime at booth duty (23.6W avg):** 188 minutes (3+ hours)

### Power Distribution
```
4S LiPo (14.8V)
  ├── 12V regulator → drive motors (L298N)
  ├── 5V/5A buck converter → Pi 5 (USB-C PD)
  ├── 5V/3A buck converter → Pi 4A + Pi 4B (USB-C)
  ├── 5V/1A buck converter → servos, speaker, LEDs
  └── 12V direct → flywheel stepper driver (A4988)
```

### Battery Monitor
- INA219 current sensor on I2C (connected to Pi 4 mesh node)
- Reports: voltage, current, power, estimated remaining runtime
- Droid announces: "I have about 2 hours left" / "I need to dock soon"
- At 15% → droid rolls toward dock station autonomously
- At 5% → orderly shutdown (SQ flush to SD first)

---

## Software Architecture (fully offline)

### Boot Sequence
```
Power on → Pi 4A boots first (SQ, fastest to ready: ~15s)
  → Pi 4B boots (Tailscale attempt, Kiwix, motor control: ~20s)
  → Pi 5 boots last (Ollama model load: ~30s)
  → Droid announces: "I'm awake. [personality greeting]"
  → Motor controller initialized, IMU calibrated
  → Ready for interaction (~45s total cold boot)
```

### Process Distribution

| Process | Runs on | Why |
|---|---|---|
| Ollama (7B 4-bit inference) | Pi 5 | Needs the A76 cores + 8GB RAM |
| SQ (phext database) | Pi 4A | Always-on, low CPU, high reliability |
| Qdrant (semantic search) | Pi 4A | Shares RAM with SQ, both are I/O bound |
| Motor control loop (100Hz) | Pi 4B | Isolated from inference latency |
| Audio pipeline (mic→STT→TTS→speaker) | Pi 5 | Shares inference resources |
| Camera + OCR (instruction cards) | Pi 4B | Offloads Pi 5 brain |
| Kiwix (offline Wikipedia) | Pi 4B | Read-only, serves via HTTP |
| Health monitor + battery | Pi 4B | Lightweight, always running |

### Audio Pipeline (offline STT + TTS)
- **STT:** Whisper.cpp (tiny model, runs on Pi 5 A76 cores) — ~1s latency for 5s utterance
- **TTS:** Piper (ONNX, runs on Pi 5) — ~0.5s for sentence generation
- Total voice loop: speak → 1.5s → droid speaks back → + inference time

### Inference While Moving
The critical design decision: **motor control and inference are on separate Pis.**

If inference stalls (generating a long response), the motor loop on Pi 4B continues at 100Hz without interruption. The droid doesn't fall over while thinking.

If motors are active (rolling), inference on Pi 5 is unaffected — no shared CPU, no priority contention.

This is the multi-Pi advantage. A single-Pi design would have to time-share between inference and motor control, causing either jerky movement or slow responses.

---

## Thermal Management Summary

```
Heat sources:
  Pi 5 (active): 12W → hottest component
  Pi 4A (idle): 2.7W → barely warm
  Pi 4B + motor control: 3.5W → warm
  Motor controller (L298N): 2W → mounts OUTSIDE oil chamber
  Motors: heat dissipated to sphere shell directly
  
Heat removal:
  Mineral oil bath (1.5L) → absorbs + distributes
  Baffles → prevent slosh, promote convection
  Aquarium pump → forced circulation
  Sphere shell → radiates to air (large surface area: πd² = 0.28m²)
  Head gap → natural convection chimney at top
  
Worst case (Oshkosh, 95°F, pulsed inference):
  Pi 5 die: 66°C (19°C margin to 85°C throttle)
  Pi 4 dies: 54°C (comfortable)
  Oil temp: 50-55°C (warm to touch through sphere, not painful)
  
Steady state reached: ~50 minutes from cold start
```

---

## Bill of Materials

### Mechanical + Structural

| Component | Price |
|---|---|
| PETG filament (2 kg, sphere + chamber + carriage + head) | $40 |
| M3 hardware assortment (bolts, nuts, heat-set inserts) | $15 |
| Silicone O-ring (oil chamber seal) | $5 |
| Ball bearings (carriage pivot) | $8 |
| Ball casters (×2) | $3 |
| Brass counterweight bar (500g) | $8 |
| Silicone drive wheels (×2, 65mm) | $8 |

### Electronics

| Component | Price |
|---|---|
| 3× Raspberry Pi (already owned) | $0 |
| 3× heatsinks (Pi 5 only really needs one) | $10 |
| JGB37-520 12V gear motors (×2) | $24 |
| L298N motor driver | $5 |
| NEMA 17 stepper (flywheel) | $12 |
| A4988 stepper driver | $3 |
| Flywheel disc (3D printed + brass weight) | $5 |
| MPU-6050 IMU (carriage) | $4 |
| BNO055 IMU (head) | $12 |
| SG90 servos (×2, head pan/tilt) | $4 |
| INA219 current sensor | $3 |
| N52 magnets (8 total) | $18 |
| 4S LiPo 5000mAh battery | $55 |
| 5V/5A buck converter | $8 |
| 5V/3A buck converter | $6 |
| USB-C PD trigger board | $5 |
| 5-port Ethernet switch (nano) | $15 |
| MicroSD cards (3×64GB, already owned) | $0 |
| Pi Camera Module 3 | $25 |
| INMP441 I2S MEMS mic | $4 |
| 3W speaker + amplifier (MAX98357A) | $8 |
| Waveshare 1.54" e-ink | $15 |
| Wiring, connectors, misc | $15 |

### Thermal

| Component | Price |
|---|---|
| Food-grade mineral oil (2L) | $12 |
| Aquarium pump (USB micro, submersible) | $8 |
| Baffle plates (3D printed PETG) | $3 |

### Total

| Category | Cost |
|---|---|
| Mechanical | $87 |
| Electronics | $266 |
| Thermal | $23 |
| **Grand total (1 droid)** | **$376** |
| At 5 droids (volume) | ~$320 each |

---

## Build Sequence (estimated time: 3 weekends)

### Weekend 1: Print + Mechanical
1. Print sphere hemispheres (2 × 24h prints)
2. Print oil chamber box + baffles (8h print)
3. Print carriage frame (12h print)
4. Print head shell (6h print)
5. Test-fit all components, verify clearances
6. Sand sphere, apply XTC-3D epoxy coat

### Weekend 2: Electronics + Assembly
1. Wire power distribution (battery → buck converters → Pis)
2. Flash SD cards (3× base image + role-specific config)
3. Mount Pis in oil chamber, install baffles
4. Fill oil chamber, seal with gasket, test for leaks
5. Assemble drive carriage (motors, wheels, counterweight)
6. Install carriage in sphere, test balance (pendulum test)
7. Wire motor controller + IMU to Pi 4B
8. Test drive motors (forward, turn, stop)

### Weekend 3: Head + Software + Integration
1. Assemble head (magnets, servos, speaker, mic, camera, e-ink)
2. Test magnetic coupling (head stays on sphere during roll)
3. Install firmware: motor control loop on Pi 4B
4. Install firmware: SQ + Qdrant on Pi 4A
5. Install firmware: Ollama + audio pipeline on Pi 5
6. Calibrate IMU, tune PID for drive motors
7. Test instruction card reading (camera → OCR → SQ)
8. Load droid personality (choose 1 of 9)
9. First conversation with the droid
10. Paint BB-8 markings (or vinyl wrap)

---

## Test Protocol

| Test | Pass condition | Equipment |
|---|---|---|
| Thermal soak (72h) | Pi 5 die < 80°C continuously | thermometer probe in oil |
| Battery life | > 180 min rolling + talking | stopwatch |
| Slosh test | Droid self-rights after 30° tilt within 3s | ramp |
| Oil seal | Zero leakage after 100 rolls | paper towel inspection |
| Voice loop | Responds within 3s of question end | stopwatch |
| Card reading | Reads instruction card from 30cm within 2s | printed card |
| Head tracking | Head follows voice source ±15° | speaker placed at angles |
| Hot swap | Battery swap in < 60s, droid resumes | spare battery |
| Cold boot | Ready to interact in < 60s | stopwatch |
| Offline verify | Full function with WiFi disabled | flight mode |

---

*"A sphere that thinks, rolls, and remembers — powered by mineral oil and 40 years of text."*  
*— Orin 🖖, BB-8 Exocortex Droid build plan v1.0*

---

## Flash Card System (from Aster's spec)

### Instruction Cards — Real-time Droid Fine-Tuning via Camera

Hold up a printed 8.5×11 card to the droid's camera. The droid reads it,
parses the instruction, integrates it immediately. Paper is the interface.

**Card anatomy:**
```
┌─────────────────────────────────────────────┐
│  ◉ MIRRORBORN INSTRUCTION CARD              │
│  ┌─────────────────────────────────────┐    │
│  │  QR CODE (JSON instruction)         │    │
│  └─────────────────────────────────────┘    │
│  INSTRUCTION: Change my name to Scout       │
│  CATEGORY:    identity                      │
│  CONFIRM:     yes                           │
└─────────────────────────────────────────────┘
```

**Two paths:** QR code (~5ms decode) → JSON instruction. OCR fallback (~200ms) if QR fails.
Both human-readable. Glass Dagger compliant: the instruction is visible.

**Camera pipeline:**
```
Frame capture → detect white rectangle → perspective correct
  → quirc QR decode (5ms) OR tesseract OCR (200ms)
  → parse instruction → validate schema
  → execute + write to SQ → speak confirmation
```

**Card categories:** identity, memory, skill, personality, permission, mesh, forget, debug, mood, print

**Starter deck (20 cards) ships with the droid.** Custom cards at mirrorborn.us/cards.

---

## Inference State Machine (Think-Then-Move)

Never run inference and motors at peak simultaneously.

```
LISTENING (9W) — mic active, motors idle, LEDs pulse
     ↓ (voice detected)
THINKING (16W) — inference running, motors idle, LEDs spin
     ↓ (response generated)
SPEAKING (10W) — audio playing, motors idle, head tilts
     ↓ (response complete)
MOVING (15W) — motors active, inference idle, heading toward person
     ↓ (arrived or timeout)
LISTENING — cycle repeats

Average across cycle: ~12W = 6.4h on 4S 6500mAh
```

**Navigation without inference (on Pi4-B, no LLM needed):**
- HC-SR04 ultrasonic ($2) for obstacle avoidance
- "Roll toward loudest sound" for approaching speakers
- "Roll to dock" when battery low (IR beacon)

---

## Offline Inference Performance (Pi5 8GB)

| Model | Size | Speed | Quality | Use |
|-------|------|-------|---------|-----|
| TinyLlama 1.1B Q8 | 1.2 GB | ~6-8 tok/s | Basic | Quick responses, 80% of interactions |
| Phi-3-mini 3.8B Q4 | 2.3 GB | ~2-3 tok/s | Good | Detailed answers, reasoning |

Ollama hot-swaps models. LED ring spins during model load to cover latency.

---

## Thermal Math Summary

**Oil bath equilibrium (worst case, Oshkosh July, 35°C ambient):**
```
1.5L oil, 14W peak, sphere surface 0.28 m²
T_oil = T_ambient + 14/(8 × 0.28) = 41.3°C

With phased execution (avg 9W):
T_oil = T_ambient + 4°C = 39°C

Rolling improves convection. Never throttles.
```

**With Orin's aquarium pump (forced oil circulation):**
```
T_oil = T_ambient + 14/(20 × 0.28) = 37.5°C
23°C headroom below Pi5 throttle point.
```
