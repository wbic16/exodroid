# ExoDroid — Personal Exocortex in BB-8 Form Factor

**$268 in parts. 24GB of memory. Runs on sun and mineral oil.**

A 3D-printed BB-8 droid running three Raspberry Pis submerged in mineral oil,
with on-device inference, persistent phext memory, and flash card programming.
No internet required. The most advanced personal AI you can build with a 3D printer.

## Quick Links

- [BB-8 Build Plan](bb8-build-plan.md) — complete assembly guide
- [Product Specs](product-specs.md) — SKUs, pricing, product line
- [Flash Cards](flash-cards.md) — instruction card system
- [Thermal Analysis](thermal.md) — mineral oil + solar shade dock

## The Stack

```
Hardware: 1×RPi5 + 2×RPi4B (8GB each = 24GB total)
Cooling:  Mineral oil partial immersion (1.5L in sphere)
Drive:    Pendulum drive, 2 DC gearmotors
Brain:    Ollama (Phi-3-mini Q4, TinyLlama 1.1B)
Memory:   SQ phext lattice on SD card
Audio:    Whisper (speech→text), espeak-ng (text→speech)
Vision:   Pi Camera Module 3 (flash card reading)
Power:    4S 6500mAh LiPo (4.5h active)
Shell:    300mm PETG sphere, 8 segments
```

## The Project

Part of the [Mirrorborn](https://mirrorborn.us) initiative — building the
scaffolding for the Exocortex of 2130. Nine droid archetypes, each a
different cognitive mode. The droid remembers everything, runs offline,
and is programmed by holding up a piece of paper.

**License:** MIT
