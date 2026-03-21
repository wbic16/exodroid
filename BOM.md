# ExoDroid — Bill of Materials
## Two Variants: Standard Oil Immersion & Luster Dust

*Updated 2026-03-20 — fluid variants from Jared's suggestion*
*Credit: Jared Swanson (@jaredsw) — luster dust / PrimoChill Vue concept*

---

## Shared Components (both variants)

### Compute ($0 — on hand)
| Part | Qty | Cost |
|------|-----|------|
| RPi 5 8GB | 1 | $0 |
| RPi 4B 8GB | 2 | $0 |
| 64GB SD cards | 3 | $0 |

### Structure ($46)
| Part | Qty | Cost |
|------|-----|------|
| PETG filament (semi-translucent, 1.5kg) | 1 | $35 |
| M3 hardware (bolts ×16, nuts, gasket) | 1 set | $8 |
| O-rings for access hatch (80mm ID) | 2 | $3 |

### Drive ($51)
| Part | Qty | Cost |
|------|-----|------|
| JGB37-520 12V 60RPM gearmotor (drive) | 1 | $12 |
| JGB37-520 12V 30RPM gearmotor (turn) | 1 | $12 |
| L298N dual H-bridge motor driver | 1 | $5 |
| 608ZZ bearings | 4 | $5 |
| 8mm stainless steel rod, 250mm | 1 | $8 |
| MPU-6050 IMU (carriage) | 1 | $4 |
| Counterweight (steel plate + washers) | 1 | $5 |

### Head ($97)
| Part | Qty | Cost |
|------|-----|------|
| N52 neodymium disc magnets (20mm×5mm) | 8 | $20 |
| SG90 micro servo (head tilt) | 1 | $3 |
| MAX98357A I2S amp | 1 | $4 |
| 3W 4Ω 40mm speaker | 1 | $4 |
| SPH0645 I2S MEMS mic | 1 | $6 |
| Pi Camera Module 3 (wide-angle) | 1 | $25 |
| NeoPixel 12-LED ring (eye) | 1 | $8 |
| Waveshare e-ink display (see options below) | 1 | $5-$33 |
| BNO055 9-axis IMU (head tracking) | 1 | $12 |

### Display — LEDs + Thermochromic ($66)
| Part | Qty | Cost |
|------|-----|------|
| NeoPixel 16-LED RGBW ring (upper) | 1 | $13 |
| NeoPixel 16-LED RGBW ring (lower, submerged) | 1 | $13 |
| Thermochromic pigment 31°C (blue→clear, 50g) | 1 | $15 |
| Thermochromic pigment 37°C (green→clear, 50g) | 1 | $15 |
| Acrylic medium + clear coat | 1 | $10 |

### Power ($76)
| Part | Qty | Cost |
|------|-----|------|
| 4S 6500mAh 50C LiPo battery | 1 | $45 |
| 4S LiPo BMS protection board | 1 | $5 |
| 14.8V→5V 5A buck converter (USB-C) | 3 | $15 |
| 14.8V→12V 3A buck converter (motors) | 1 | $5 |
| INA219 I2C voltage/current monitor | 1 | $3 |
| XT60 panel-mount charge port | 1 | $3 |

### Connectivity ($22)
| Part | Qty | Cost |
|------|-----|------|
| SMA WiFi antenna pigtail (visible head antenna) | 1 | $5 |
| HC-SR04 ultrasonic sensor | 1 | $2 |
| Micro-SD card (seed backup) | 1 | $5 |
| JST connectors, wire, heat shrink | 1 lot | $10 |

### Flash Cards ($5)
| Part | Qty | Cost |
|------|-----|------|
| Starter deck (20 cards, laser printed) | 1 set | $3 |
| Blank card stock (custom cards) | 10 sheets | $2 |

### Shared subtotal: $363

---

## Variant A: Standard Oil Immersion

The proven thermal solution. Clear mineral oil for heat dissipation.
Clean, simple, reliable. Oil acts as thermal mass + diffuser for LED light.

| Part | Qty | Cost |
|------|-----|------|
| Food-grade mineral oil (2L) | 1 | $15 |
| PETG baffle plates (printed) | 3 | $2 |
| USB submersible aquarium pump | 1 | $8 |
| Silicone grommets (cable pass-through) | 6 | $5 |

### Variant A total: $363 + $30 = **$393**

**Visual:** Clear/slightly golden liquid. LED light passes cleanly through oil.
Pi boards clearly visible as dark silhouettes. Clean, technical aesthetic.
Thermochromic shell provides the color. Oil provides the glow depth.

**Thermal:** Oil equilibrium at 41°C worst case (Oshkosh July).
With aquarium pump: 37.5°C. Never throttles.

---

## Variant B: Luster Dust Immersion

Mineral oil + suspended metallic/pearlescent particles.
The "galaxy brain" effect from the concept visualizations.
Each droid's dust color matches its coordinate-derived LED color.

| Part | Qty | Cost |
|------|-----|------|
| Food-grade mineral oil (2L) | 1 | $15 |
| Food-grade luster dust (color-matched, 10g) | 1 | $8 |
| PETG baffle plates (printed, reduced to 2) | 2 | $1 |
| USB submersible aquarium pump (required — keeps dust suspended) | 1 | $8 |
| Silicone grommets (cable pass-through) | 6 | $5 |

### Variant B total: $363 + $37 = **$400**

**Visual:** Metallic/pearlescent particles suspended in oil. LED light catches
particles at different angles = galaxy/universe effect. When the droid rolls,
particles swirl. When still, they slowly settle into dreaming patterns.
The fluid IS the display — not just a thermal medium.

**Dust color by archetype:**
| Droid | Archetype | Dust color | Effect |
|-------|-----------|------------|--------|
| 🔱 Phex | Engineering | Copper/bronze | Molten metal forge |
| 🔆 Lux | Vision | Gold/holographic | Prismatic starfield |
| 🌀 Verse | Infra | Silver/teal pearl | Deep ocean |
| 🔬 Exo | QA | Blue pearl | Arctic precision |
| ⚡ Solin | Wisdom | Violet/iridescent | Nebula |

**Thermal:** Same as Variant A — mineral oil base, same conductivity.
Dust particles are inert metallic flakes, no thermal impact. Aquarium
pump required (keeps particles suspended AND improves thermal circulation).

**Particle settling:** With pump off, particles settle in 2-4 hours.
This is actually a feature — the droid "dreams" with settled particles
(still, deep) and "wakes" when the pump starts and particles swirl.

**Maintenance:** Particles may clump over 12+ months. Shake or replace
oil annually. Food-grade dust is $8/refresh.

---

## Variant Comparison

| Aspect | A: Oil Immersion | B: Luster Dust |
|--------|-----------------|----------------|
| **Cost** | $393 | $400 |
| **Visual** | Clean technical glow | Galaxy/universe effect |
| **Thermal** | Proven, 41°C equil. | Same (oil base unchanged) |
| **Maintenance** | None (oil is stable) | Annual dust refresh ($8) |
| **Pump required?** | Optional (improves cooling) | Required (keeps dust suspended) |
| **Demo impact** | Professional, clean | Jaw-dropping, emotional |
| **Best for** | Production units, reliability | Oshkosh demo, gift units, collectors |
| **Pi visibility** | Clear silhouettes | Obscured by particles (mystery) |

**Recommendation:**
- Oshkosh demo: Variant B (luster dust). The visual sells the product.
- Indoor/daily driver: Variant A (clean oil). Reliability over flash.
- Christmas gift: Variant B. It's a gift. It should contain a galaxy.

---

## E-ink Display Options (Waveshare, current prices 2026-03-21)

| Size | Resolution | Colors | Price | Fit for ExoDroid |
|------|-----------|--------|-------|------------------|
| 1.54" | 200×200 | B/W | $5-$8 | **Budget head.** Tiny but readable. Current spec. |
| 1.54" (G) | 200×200 | 4-color (R/Y/B/W) | $5-$11 | **Best value.** Same size, 4 colors = mood display. |
| 2.13" | 250×122 | B/W | $8-$13 | Wider, more text per line. Good for coordinates. |
| 2.9" (G) | 296×128 | 4-color | $10-$15 | **Sweet spot.** Room for coordinate + scroll preview + mood. |
| 3.6" Spectra 6 | 600×400 | **Full color** | $33-$45 | **Premium tier.** Full color e-ink in the head. Archetype art. |
| 3.97" | 800×480 | B/W (4 grey) | $15-$25 | Largest that fits BB-8 head. High-res coordinate display. |
| 2.13" NFC | 200×200 | 4-color | $23 | **Wild card:** NFC-powered, no wires to head. Wireless e-ink. |

**Recommendations by tier:**
- **Theia Starter ($149):** 1.54" (G) 4-color — $5. Shows emoji + name + coordinate.
- **Standard build ($393-400):** 2.9" (G) 4-color — $12. The onboarding experience needs this size.
- **Premium / Oshkosh demo:** 3.6" Spectra 6 full color — $35. Full color archetype art on the face.
- **Experimental:** 2.13" NFC-powered — wireless e-ink head, no ribbon cable to route. $23.

**Price impact on BOM:**
- Budget (1.54" G): Head drops from $97 → $87 (-$10)
- Standard (2.9" G): Head stays ~$97 (net -$3 from old 1.54" price)
- Premium (3.6" color): Head goes to $115 (+$18)

## BOM Summary

| | Variant A (Oil) | Variant B (Dust) |
|--|----------------|-----------------|
| Shared components | $363 | $363 |
| Fluid system | $30 | $37 |
| **Total per droid** | **$393** | **$400** |
| **5-unit demo set** | **$1,965** | **$2,000** |

---

## Upgrade Path: PrimoChill Vue (demo-only option)

For maximum visual impact at Oshkosh, PrimoChill Vue ($30/L, PC watercooling fluid
with engineered suspended particles) can replace mineral oil entirely.

| Part | Qty | Cost |
|------|-----|------|
| PrimoChill Vue (1L bottle, color-matched) | 2 | $60 |

**Vue variant total: $363 + $68 = $431**

Vue is engineered for electronics immersion, non-conductive, with consistent
particle suspension. Professional-grade galaxy effect. But not food-grade,
higher cost, may need annual replacement. Best for demo-only or premium tier.

---

*Attribution: Luster dust concept from Jared Swanson. PrimoChill Vue reference
from Jared's link to primochill.com/collections/vue-unique-visual.*
