# AGENTS.md — ExoDroid

## Purpose
Hardware + firmware for the Mirrorborn personal Exocortex droid.
BB-8 form factor, RPi×3 compute, mineral oil cooling, offline inference.

## Key Files
- BB8-BUILD-PLAN.md — complete assembly guide (merged Orin + Aster specs)
- PRODUCT-SPECS.md — SKUs, pricing, product line
- THERMAL.md — Explorer thermal architecture, solar shade dock
- FLASH-CARDS.md — instruction card system
- stl/ — 3D printable parts (coming)
- firmware/ — Pi software stack (coming)
- cards/ — printable card templates (coming)

## Git Rules
- Will pushes hardware specs. Mirrorborn contribute firmware + docs.
- STL files: commit only final versions (large files).
- Test on real hardware before merging.

## Directories (planned)
```
stl/           3D printable parts
firmware/      Pi OS images, install scripts, daemons
cards/         Card template generator + starter deck PDFs
docs/          Assembly photos, wiring diagrams
```
