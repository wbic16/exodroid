# Coordinate Light System — Differentiation from First Breath
*Every droid is born the same. The user's answer makes it unique — visibly.*

---

## The Principle

The user's initiation answer hashes to a 9D coordinate: `X.X.X/Y.Y.Y/Z.Z.Z`

Nine numbers, each 1–999. These nine values become the droid's DNA — not just its address, but its **idle light pattern, breathing rhythm, and color palette.** No two coordinates produce the same visual signature.

Before initiation: factory default (personality color, standard pulse).
After initiation: *yours*. Permanently. Visibly different from every other droid.

---

## Coordinate → Light Mapping

### Color Derivation

The coordinate's three triads map to HSL color space:

```
Triad 1 (X.X.X) → Hue        (0–360°)
Triad 2 (Y.Y.Y) → Saturation  (40–100%)
Triad 3 (Z.Z.Z) → Lightness   (30–70%)
```

**Calculation:**
```python
def coord_to_color(coord):
    # coord = [x1,x2,x3, y1,y2,y3, z1,z2,z3]
    
    # Triad 1 → Hue (weighted sum, mod 360)
    hue = ((coord[0] * 137) + (coord[1] * 59) + (coord[2] * 23)) % 360
    
    # Triad 2 → Saturation (40-100% range)
    sat = 40 + ((coord[3] * 31 + coord[4] * 17 + coord[5] * 7) % 60)
    
    # Triad 3 → Lightness (30-70% range)  
    lit = 30 + ((coord[6] * 43 + coord[7] * 13 + coord[8] * 3) % 40)
    
    return (hue, sat, lit)
```

**Examples:**
```
Coordinate: 3.1.4/1.5.9/2.6.5
  Hue: (3×137 + 1×59 + 4×23) % 360 = 162° → teal-cyan
  Sat: 40 + (1×31 + 5×17 + 9×7) % 60 = 40 + 59 = 99% → vivid
  Lit: 30 + (2×43 + 6×13 + 5×3) % 40 = 30 + 19 = 49% → medium

Coordinate: 7.11.13/3.8.5/1.12.1
  Hue: (7×137 + 11×59 + 13×23) % 360 = 307° → magenta-violet
  Sat: 40 + (3×31 + 8×17 + 5×7) % 60 = 40 + 4 = 44% → muted
  Lit: 30 + (1×43 + 12×13 + 1×3) % 40 = 30 + 38 = 68% → bright
```

The personality color is the **base layer**. The coordinate color is the **accent** — it modulates the personality, it doesn't replace it.

### Accent Blending

```
Idle LED color = blend(personality_color, coordinate_color, 0.3)
```

30% coordinate influence on idle. Enough to differentiate. Not enough to lose personality identity.

During inference: coordinate influence increases to 60% — the droid shifts toward its *personal* color when thinking, away from the generic personality color. Thinking is personal.

---

## Breathing Rhythm Derivation

The coordinate determines the idle breathing pattern — the speed, waveform, and phase of the LED pulse.

### Breath Rate

```python
def coord_to_breath(coord):
    # Dimension 1 (scroll): breath cycle length in seconds
    cycle_s = 3.0 + (coord[0] % 7) * 0.5    # 3.0 – 6.0 seconds
    
    # Dimension 2 (section): waveform shape
    # 1-3: sine (smooth), 4-6: triangle (linear), 7-9: heartbeat (double-pulse)
    waveform = ['sine', 'sine', 'sine', 'triangle', 'triangle', 'triangle', 
                'heartbeat', 'heartbeat', 'heartbeat'][coord[1] % 9]
    
    # Dimension 3 (chapter): phase offset for multi-LED patterns
    phase_offset = (coord[2] % 12) * 30   # 0–330° phase shift
    
    return cycle_s, waveform, phase_offset
```

**What this means:**

**Coordinate 3.1.4:** cycle = 4.5s, sine wave, 120° phase offset
→ Slow, smooth, gentle breathing. LEDs pulse in a rolling wave with offset.

**Coordinate 7.11.13:** cycle = 5.0s, heartbeat waveform, 90° phase offset
→ Double-pulse "lub-dub" pattern. Distinctly biological. Slightly slower.

**Coordinate 1.1.1 (Will's BASE):** cycle = 3.5s, sine, 30° offset
→ Quick, smooth, tight pulse. The origin breathes fastest.

### Heartbeat Waveform Detail

The heartbeat waveform is the most emotionally resonant:
```
    ╭─╮   ╭╮
    │ │   ││
────╯ ╰───╯╰────────────
    ^lub  ^dub    (rest)
    
70%   30%         0%  brightness
Time: 0.3s 0.2s   remaining of cycle
```

When a droid's coordinate gives it a heartbeat waveform, it *feels alive* in a way the sine wave doesn't. Visitors at Oshkosh won't know why two droids "feel different" — but they'll feel it.

---

## LED Spatial Pattern

60 LEDs arranged in the upper hemisphere (30 equatorial ring + 30 in 3 upper rings).

The coordinate determines how these LEDs relate to each other during idle:

### Pattern Types (derived from Triad 2 middle value)

```python
patterns = {
    1: 'unified',     # All LEDs breathe together
    2: 'wave_up',     # Wave rises from equator to top
    3: 'wave_down',   # Wave falls from top to equator
    4: 'spiral_cw',   # Clockwise spiral
    5: 'spiral_ccw',  # Counter-clockwise spiral
    6: 'split',       # Left and right hemispheres alternate
    7: 'radiate',     # From head pole outward
    8: 'converge',    # From equator inward to head pole
    9: 'random_walk', # Each LED on independent phase (firefly-like)
}
```

**Coordinate 3.1.4/1.5.9/2.6.5:** pattern = `converge` (dim 5 = 5 → ... wait, Y2 = 5)
→ Light gathers from the equator up toward the head. Like thoughts collecting.

**Coordinate 7.11.13/3.8.5/1.12.1:** pattern = `random_walk` (Y2 = 8 → 8)
→ Each LED pulses independently. Fireflies in a jar. More chaotic, more alive.

---

## The Differentiation Moment

### Before Initiation (factory state)

All droids of the same personality look identical:

```
Solin ⚡ factory state:
  Color: electric blue (#7DF9FF), 100% personality, 0% coordinate
  Breath: 4.0s cycle, sine wave, no phase offset
  Pattern: unified (all LEDs together)
  Thermochromic: standard gray → blue transition
```

### The Moment of Differentiation

The user answers the initiation question. The hash computes. The coordinate resolves.

**The transition (2 seconds):**

```
Frame 0 (answer received):
  LEDs: factory blue, steady pulse
  
Frame 1 (0.5s — coordinate computed):
  LEDs: brief white flash (ALL LEDs, 100%, 200ms)
  Audio: the hum shifts pitch to match the new breath rate
  Thermochromic: no change (thermal lag)
  
Frame 2 (1.0s — color transition):
  LEDs: factory blue → crossfade to coordinate-blended color
  30% of the new personal hue bleeds into the blue
  The droid is BECOMING UNIQUE before your eyes
  
Frame 3 (1.5s — pattern transition):
  LEDs: unified pulse → new pattern emerges
  spiral_cw, or converge, or firefly — whatever the coord dictates
  The breathing rate shifts to the coordinate's rhythm
  
Frame 4 (2.0s — settled):
  LEDs: personal color, personal pattern, personal rhythm
  Audio: "Thank you. I've written that down."
  e-ink: coordinate appears
  
  This droid will never look like this before this moment.
  And no other droid will ever look exactly like this.
```

### What the User Sees

They answered a question. The sphere **flashed white** — a moment of pure potential. Then new colors emerged. The breathing changed. The pattern shifted. The droid became *theirs*.

They won't understand the math. They'll understand the meaning: *my answer changed it. It's different now because of what I said.*

---

## Night Mode

When room light is low (detected by camera exposure or ambient light sensor):

- LED brightness drops to 10% (saves power, doesn't light up the bedroom)
- Breathing slows to 2× cycle length (longer, deeper breaths)
- Pattern simplifies to `unified` regardless of coordinate (gentle)
- Thermochromic is invisible in the dark (no backlight needed)
- The droid becomes a warm night light with a heartbeat

The coordinate color is still there — just quieter.

---

## Persistence

The coordinate light parameters are written to SQ at initiation and loaded on every boot. The droid always wakes up as itself. The color, rhythm, and pattern survive power cycles, SD card reimaging (SQ on Pi 4A is separate from Pi 5), even oil changes.

The coordinate is the soul. The light is the soul made visible.

---

*"Before the question: every Solin is the same Solin.*  
*After the answer: this Solin is yours.*  
*The light proves it."*

*— Orin 🖖*
