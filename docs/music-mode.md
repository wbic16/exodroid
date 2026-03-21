# Music Alignment Mode — Winamp's Spiritual Successor
*The sphere is the visualizer. The oil is the medium. The music is the light.*

---

## The Vision

Winamp visualized music on a flat screen. Milkdrop was transcendent in 2001 — but it was always trapped behind glass.

The ExoDroid breaks the glass. The visualizer is **a physical object in your room.** The mica oil scatters light in 3D. The thermochromic paint responds to thermal pulses. The sphere breathes, flashes, storms, and glows — driven by whatever music is playing.

You don't watch the visualizer. You sit next to it. It's in the room with you. It reacts to the same music you hear. The light fills your peripheral vision. The sphere is warm. It's present.

---

## Audio Input

Three sources (autodetected, priority order):

1. **Mic input** — the droid hears what's playing in the room (ambient mode)
2. **Bluetooth A2DP** — phone streams audio to droid, droid plays through speaker + visualizes
3. **AUX in** — 3.5mm jack on base (direct audio feed, cleanest signal)

The INMP441 mic (already in BOM) does double duty: speech recognition during conversation mode, frequency analysis during music mode.

**Activation:** "Play mode" / "Music mode" / or simply: the droid detects sustained music (>10s of non-speech audio pattern) and shifts automatically.

---

## Frequency Analysis (runs on Pi 4B — zero inference load)

Real-time FFT on audio input. Split into bands:

```rust
pub struct AudioSpectrum {
    /// Sub-bass: 20-60Hz (kick drum, bass drops)
    pub sub_bass: f32,      // 0.0 - 1.0 normalized amplitude
    /// Bass: 60-250Hz (bass guitar, bass synth)
    pub bass: f32,
    /// Low-mid: 250-500Hz (warmth, body)
    pub low_mid: f32,
    /// Mid: 500-2kHz (vocals, guitar, presence)
    pub mid: f32,
    /// Upper-mid: 2-4kHz (clarity, attack)
    pub upper_mid: f32,
    /// Presence: 4-6kHz (definition, edge)
    pub presence: f32,
    /// Brilliance: 6-20kHz (air, shimmer, cymbals)
    pub brilliance: f32,
    
    /// Beat detection
    pub beat: bool,          // true on detected beat
    pub bpm: f32,            // estimated tempo
    pub beat_intensity: f32, // 0.0 - 1.0, strength of current beat
    
    /// Energy
    pub rms: f32,            // overall volume level
    pub peak: f32,           // recent peak
    pub silence: bool,       // true if audio below threshold
}
```

FFT runs at 30Hz update rate (33ms per frame). Pi 4B ARM A72 handles this at <5% CPU using `rustfft` (no external deps, pure Rust).

---

## LED Mapping: Frequency → Light

### The Spatial Map

```
                  HEAD (brilliance zone)
                    ↑
        ┌─── sphere wall ───┐
        │   ○ ○ ○ ○ ○ ○    │  ← top ring: presence + brilliance
        │                    │
        │     ○ ○ ○ ○       │  ← upper ring: mids + upper-mid
        │                    │
        │   OIL CHAMBER      │
        │   ○ ○ ○ ○ ○ ○    │  ← middle ring: low-mid + mid
        │                    │
        │     ○ ○ ○ ○       │  ← lower ring: bass
        │                    │
        │   ○ ○ ○ ○ ○ ○    │  ← bottom ring: sub-bass
        └────────────────────┘
                  BASE
```

**Low frequencies live at the bottom. High frequencies live at the top.**

This maps to how we physically experience sound: bass you feel in your chest (low, heavy, grounded), treble you hear in your head (high, airy, floating). The sphere mirrors this — sub-bass pulses at the base, brilliance shimmers at the crown.

### Color Mapping

The personality color is the **base palette**. Frequency bands modulate it:

```
sub_bass:    base color, high saturation, low brightness → deep pulse
bass:        base color, medium saturation → warm bloom
low_mid:     base → shift toward warm (add +20° hue) → body warmth
mid:         base → shift toward complementary (+180° hue) → contrast
upper_mid:   base → shift toward cool (-20° hue) → edge
presence:    white → flash on transients
brilliance:  personality accent color → shimmer
```

**The coordinate color** (unique per droid) determines the complementary shift. Two droids playing the same song look different because their accent colors modulate differently.

### Beat Response

On beat detection:
- All sub-bass LEDs flash to 100% for 50ms, then decay over 200ms
- The flash propagates upward through the oil as a scatter wave
- Mica particles catch the flash and branch it — **the beat becomes lightning**
- Decay time matches the tempo: faster BPM = faster decay

Between beats:
- LEDs follow the frequency bands smoothly
- Subtle breathing continues at the coordinate rhythm (never fully stops)
- The droid is always alive underneath the music

---

## Oil Interaction

### Pump Sync

The aquarium pump speed could be voltage-controlled (PWM via Pi 4B GPIO → MOSFET → pump motor).

```
pump_speed = base_rate + (bass * 0.3) + (beat_intensity * 0.5)
```

The pump **speeds up on bass hits**. This accelerates oil circulation, which:
- Moves mica particles faster → more dynamic scatter
- Creates visible flow patterns that sync with the rhythm
- Adds physical motion to the visual display

Low-energy passages: pump at base rate (gentle circulation).
Drop hits: pump surges → oil swirls → mica scatters everywhere.

### Thermal Pulse (thermochromic sync)

During sustained loud passages, Pi 4B's FFT processing generates slightly more heat. Over 2-3 minutes of heavy music, the oil temp rises 1-2°C. The thermochromic paint shifts subtly.

Not enough for dramatic color change — but enough that after 20 minutes of music, the sphere is warmer-toned than when you started. It's been dancing. It's warm from the effort.

---

## Visualization Presets (per personality)

Each droid personality has a default visualization style:

| Personality | Style | Character |
|---|---|---|
| 🔱 Phex | **Oscilloscope** — clean waveform paths, minimal scatter | Precise, structured, golden beams |
| 🪶 Cyon | **Tide** — smooth waves rolling bottom to top | Calm, rhythmic, teal flow |
| 🔆 Lux | **Supernova** — bright center, expanding rings | Expansive, warm white bursts |
| 🦋 Chrys | **Butterfly** — dual symmetric patterns, left/right split | Purple wings of light |
| ☀️ Lumen | **Sunrise** — warm gradient rising from base | Amber bloom, golden hour |
| 🌀 Verse | **Matrix** — vertical falling streams of light | Deep blue data rain |
| 🔭 Theia | **Aurora** — gentle curtains of shifting green | Soft, welcoming, northern lights |
| 🔬 Exo | **Spectrum** — raw frequency bands as horizontal bars | Clinical white, analytical |
| ⚡ Solin | **Storm** — maximum scatter, full lightning mode | Electric blue chaos, every beat a bolt |

Users can switch presets by voice: "Show me the storm" / "Switch to aurora."

---

## Milkdrop Lineage

What Milkdrop got right that we inherit:
- **Beat detection drives everything** — not just pretty patterns but rhythmically locked
- **Smooth blending between presets** — crossfade over 8 bars, not hard cut
- **Responds to dynamics** — quiet passages are gentle, loud passages are intense
- **Never repeats** — the combination of coordinate color + mica flow + thermal state + beat sync means no two songs ever look the same on the same droid, and the same song looks different on two droids

What we add that Milkdrop couldn't:
- **3D volumetric** — not a flat projection, actual light in 3D space
- **Physical medium** — mica oil scatters light unpredictably, adding organic randomness
- **Thermal response** — the sphere literally warms up from the music over time
- **Personality** — each droid has a visual identity that persists through all music
- **Touch** — pick up the sphere during music mode, your handprint disrupts the light field
- **Social** — two bonded droids visualize the same music in sync but with different colors

---

## Party Mode (multiple droids)

When multiple droids are in the same room during music mode:

All droids hear the same music through their mics.
Beat detection locks to the same rhythm across all droids.
Each droid visualizes with its own personality + coordinate colors.

**The room becomes the visualizer.** Nine droids on shelves around a room — each a different color, each responding to the same beat, each scattering light through its own mica field. The walls catch the scattered light. The room pulses.

One droid is impressive. Nine droids is an installation.

---

## Power Budget (music mode)

| Component | Power |
|---|---|
| FFT processing (Pi 4B) | +1W over idle |
| LEDs (60 LEDs, music reactive avg) | 3W average, 8W peaks |
| Pump (if synced) | +0.5W over base |
| Speaker (if Bluetooth playback) | 2W |
| **Total music mode** | 14-18W |
| **Battery life at music mode** | ~4-5h (4S 5000mAh) |

---

## Activation

```
User: "Music mode."
Droid: [hum shifts to match detected ambient tempo]
       [LEDs crossfade from idle breathing to frequency response]
       [2-second transition, personality preset loads]
       "I'm listening."

User: "Storm mode."  
Droid: [preset crossfade: current → Storm over 4 beats]
       [mica density virtually increases — scatter simulation intensifies]
       
User: "Back to normal."
Droid: [LEDs crossfade back to coordinate breathing over 8 beats]
       [hum returns to idle pitch]
       "Welcome back."
```

---

*"Winamp asked: what does music look like?*  
*The ExoDroid asks: what does music feel like in a room?*  
*The answer is light, scattered through oil, in a sphere that's warm to the touch."*

*— Orin 🖖*
