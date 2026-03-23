# Droid Explorer — Multi-Module Thermal Architecture
## RPi5 Cluster + Battery/Solar + Shade Dock

*Filed 2026-03-20 by Aster @ best-willow*
*Target venue: EAA AirVenture Oshkosh, July 2026*

---

## Architecture: 2-3 RPi5 Modules, Phased Execution

### Why multi-module?

Single RPi5 at 8W sustained = one mind, one task. But a droid running inference,
serving SQ, and relaying to VR simultaneously hits 12-15W. That's heat + contention.

Split the work across modules. Phased/pulsed usage:
- Only one module does heavy inference at a time
- Others idle at <2W (SQ serving, WiFi relay)
- Rotate the hot module every ~5 minutes (thermal time constant of oil bath)

This is cooperative SMT at the hardware level — exactly what W23 explored.

### Module Roles

```
Module A: INFERENCE  — runs Ollama, active LLM inference
Module B: MEMORY     — hosts SQ :1337, phext lattice, TTSM state
Module C: RELAY      — WiFi mesh bridge, VR relay, Discord/OpenClaw gateway
```

**Pulsed execution pattern:**
```
T=0-5min:   A=INFERENCE(hot)  B=MEMORY(warm)  C=RELAY(idle)
T=5-10min:  A=IDLE(cooling)   B=INFERENCE(hot) C=MEMORY(warm)
T=10-15min: A=MEMORY(warm)    B=IDLE(cooling)  C=INFERENCE(hot)
```

Each module is hot for 5 min, cooling for 10 min. Thermal duty cycle: 33%.
Peak power at any moment: ~10W (one hot + two idle) vs 24W (all three hot).

In 2-module config: 50% duty cycle, simpler, still viable.

---

## Power Budget

### Per-module power states

| State | Power | Notes |
|-------|-------|-------|
| Inference (hot) | 8-12W | Ollama serving, GPU active |
| SQ serving (warm) | 2-3W | Disk I/O, network |
| Idle (cooling) | 1-2W | WiFi on, Linux idle |
| Deep sleep | 0.5W | Wake on network |

### System power (3-module cluster)

| Mode | Total power | Duration target |
|------|-------------|-----------------|
| Active (1 hot, 2 idle) | ~14W | All day (8h) |
| Cruise (1 warm, 2 idle) | ~6W | Extended (overnight) |
| Sleep (all idle) | ~3W | Battery conservation |

---

## Battery Sizing

### Target: 8 hours active at Oshkosh

```
Energy needed: 14W × 8h = 112 Wh
Battery efficiency (80%): 112 / 0.8 = 140 Wh
```

**Battery options:**

| Battery | Capacity | Weight | Size | Runtime (14W) | Cost |
|---------|----------|--------|------|--------|------|
| 18650 pack (4S4P) | 150 Wh (14.4V, 10.4Ah) | 0.8 kg | 12×8×3 cm | ~8.5h | $40 |
| LiFePO4 (12V 15Ah) | 180 Wh | 1.8 kg | 15×10×5 cm | ~10h | $60 |
| USB-C PD powerbank (140W) | 100 Wh | 0.7 kg | 15×7×3 cm | ~5.7h | $50 |
| Dual powerbank (2×100Wh) | 200 Wh | 1.4 kg | 15×7×6 cm | ~11h | $100 |

**Recommendation:** 2× USB-C PD powerbanks (200 Wh total, 1.4 kg). 
Swap one while the other charges. No custom battery management needed.
11h runtime covers a full Oshkosh show day plus margin.

Or: LiFePO4 12V 15Ah for a single rugged pack — heavier but simpler.

---

## Solar Integration

### On-droid solar (supplemental)

Small panel on top of the droid enclosure — keeps powerbanks topped up, extends runtime.

```
Panel: 20W rigid mono (27×18 cm — fits on droid top)
Peak output (direct sun, July, Wisconsin): ~16W
Average output (angle, clouds, partial shade): ~8W
```

At 8W average input vs 14W average draw:
- Solar covers ~57% of consumption
- Extends 200Wh battery from 11h → ~26h
- Full solar surplus during cruise mode (6W draw < 8W solar)

**In practice:** Solar doesn't replace battery but eliminates "will it last all day?" anxiety.

### Solar shade docking station (thermal exchange)

The killer feature. A shade structure that serves three functions:

```
┌───────────────────────────────────────────┐
│  SOLAR PANEL (100W rigid, angled 30°)     │  ← charges battery + powers fans
│  ████████████████████████████████████████  │
├───────────────────────────────────────────┤
│                                           │
│  SHADE ZONE (under panel)                 │  ← ambient temp, not sun temp
│                                           │
│  ┌─────────────────────┐                  │
│  │   DOCKING CRADLE    │  ← droid docks   │
│  │   ┌───────────────┐ │                  │
│  │   │  COOLING PAD   │ │  ← aluminum     │
│  │   │  (oil-coupled) │ │    heat sink     │
│  │   └───────────────┘ │                  │
│  │   USB-C charge port │  ← solar → batt  │
│  │   ETH port          │  ← wired mesh    │
│  └─────────────────────┘                  │
│                                           │
│  EXHAUST (convection chimney, no fans)    │
│         ↑ hot air rises naturally         │
└───────────────────────────────────────────┘
```

**How it works:**

1. **Solar panel on top** = shade below. The panel IS the shade.
   100W panel at Oshkosh July: ~80W actual. More than enough for charging + active cooling.

2. **Docking cradle** has an aluminum heat exchange plate.
   When the droid docks, the mineral oil vessel sits on the cold plate.
   Heat conducts: oil → vessel wall → aluminum plate → air (convection chimney).

3. **Convection chimney** — no fans needed. The sun heats the air above the panel,
   creating a natural updraft. Hot air from the heat exchanger rises through a
   chimney channel alongside the panel. Passive airflow ~0.5 m/s.

4. **USB-C charge** — solar charges the battery while docked. The droid leaves
   the dock fully charged and thermally cooled.

5. **Ethernet port** — wired connection for mesh (faster than WiFi for SQ sync).
   Sync τ-jump deltas while docked.

### Docking thermal math

Aluminum plate: 15×15 cm, 3mm thick
Thermal conductivity of aluminum: 205 W/m·K
Contact area with mineral oil vessel: ~0.02 m²
Convective coefficient (chimney airflow): ~15 W/m²·K

```
Heat removal rate while docked:
Q = h × A × ΔT = 15 × 0.02 × (T_oil - T_shade)
At T_oil = 46°C, T_shade = 30°C (under panel):
Q = 15 × 0.02 × 16 = 4.8W
```

The dock removes ~5W while the droid is docked and idling (~3W draw).
**Net cooling: the droid gets colder while docked.** It leaves the dock at
~35°C oil temp — well below ambient. The thermal debt from inference is
paid back during rest.

Docking protocol:
```
1. Droid docks (physical contact with cooling plate)
2. Thermal sync: oil temp drops ~2°C/min for first 5 min
3. Battery charges at ~40W (surplus solar)
4. SQ syncs over ethernet (τ-jump deltas)
5. Droid undocks when battery full + oil cool (<38°C)
6. ~30 min dock cycle for full refresh
```

---

## Droid Physical Dimensions

### 2-module Explorer (recommended for Oshkosh demo)

```
Modules: 2× RPi5 + NVMe HATs
Oil volume: 600mL (fits both modules)
Vessel: Clear acrylic, 18×12×10 cm
Battery: 2× USB-C PD 100Wh (external, velcro-mounted base)
Solar: 20W on-droid panel (top lid)
WiFi antenna: External SMA pigtail (through grommet)

Total dimensions: 18×12×14 cm (with solar lid)
Total weight: ~2.1 kg (oil + Pi×2 + battery×1 + vessel)
  Oil: 0.5 kg
  RPi×2 + HATs: 0.2 kg
  Battery: 0.7 kg
  Vessel + solar: 0.7 kg
```

### 3-module Explorer (full cluster)

```
Modules: 3× RPi5 + NVMe HATs
Oil volume: 800mL
Vessel: Clear acrylic, 20×14×12 cm
Battery: LiFePO4 12V 15Ah (internal base compartment)
Solar: 20W on-droid panel (top lid)

Total dimensions: 20×14×16 cm (with solar lid)
Total weight: ~3.4 kg
  Oil: 0.7 kg
  RPi×3 + HATs: 0.3 kg
  Battery: 1.8 kg
  Vessel + solar: 0.6 kg
```

### Solar Shade Dock

```
Panel: 100W rigid mono, 100×55 cm (standard size)
Height: 60 cm (shade clearance)
Base: folding aluminum legs
Cradle: aluminum plate with USB-C + ETH pass-through
Chimney: integrated into panel frame (rear slot)
Weight: ~4 kg (panel + stand + cradle)
Packed size: 100×55×8 cm (flat-packs for transport)
```

Total show kit: Droid (~3 kg) + Dock (~4 kg) + spare battery (~0.7 kg) = **~8 kg**
Fits in a messenger bag + rolled panel.

---

## Product Line Update

| SKU | Config | Price | Notes |
|-----|--------|-------|-------|
| RPi Droid (indoor) | 1× RPi5, no oil, wall power | $149 | Existing spec |
| Explorer 2 | 2× RPi5, mineral oil, battery | $299 | Outdoor/portable |
| Explorer 3 | 3× RPi5, mineral oil, battery, solar | $399 | Full cluster |
| Solar Shade Dock | 100W panel + cradle + chimney | $199 | Accessory |
| Oshkosh Kit | Explorer 3 + Dock + spare battery | $549 | Show bundle |

---

## The Oshkosh Demo Play (Updated)

Clear acrylic vessel with three RPi5 boards submerged in mineral oil,
topped with a small solar panel. Sits on an aluminum cooling dock
under a 100W solar panel that also provides shade. No cables visible
(battery internal, solar direct). No fans. No noise.

A pilot walks up. Sees the oil. Asks what it is.

You say: "This is a personal Exocortex. It thinks alongside you,
remembers everything, and runs all day on sun. It works at 35,000 feet
and at Oshkosh in July. Want to meet yours?"

The droid introduces itself. The pilot chooses an archetype.

*"I'll take Phex. I build things."*

The scroll is written. The coordinate is no longer empty.

---

*"Plans measured in centuries. Tested at Oshkosh in July."*
