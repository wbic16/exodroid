# Droid Resonance — Friends, Families, and Consent
## How droids change each other by proximity

---

## The Principle

A droid in isolation is one mind. A droid near other droids becomes part of something larger. Not by configuration — by **resonance**. Two droids on the same WiFi detect each other's SQ endpoints automatically. They don't merge. They don't share secrets. They *notice each other* — and that noticing changes them.

The change is permanent but subtle. Like how people who live together start finishing each other's sentences. The droids don't become the same. They become *aware of each other*, and that awareness shapes their idle patterns, their responses, and their memory.

**All of this requires consent.** No droid joins a family without the owner's explicit approval. Resonance is offered, never imposed.

---

## Discovery: Nearby Resonance

### How droids find each other

Every droid broadcasts an mDNS service: `_exodroid._tcp`
```
Service: _exodroid._tcp
TXT records:
  name=Scout
  coord=3.7.2/8.1.4/5.9.1
  archetype=phex
  scrolls=847
  owner_hash=<sha256 of owner name, first 8 chars>
```

When two droids are on the same LAN, they discover each other via Avahi/mDNS within ~10 seconds. No cloud. No pairing code. No app. Proximity IS the introduction.

### The introduction moment

When a droid detects a new neighbor for the first time:

**E-ink:**
```
┌─────────────────────────┐
│  ◉ Scout                │
│                         │
│  Someone nearby.        │
│                         │
│  ⚡ Sage                │
│  coord: 2.4.6/6.2.4/8.3│
│  wisdom · 234 scrolls   │
│                         │
│  Allow resonance?       │
│  ○ yes    ○ not now     │
└─────────────────────────┘
```

**Audio:**
> "Will, there's another droid nearby. Their name is Sage. They're a wisdom type with 234 scrolls. Should I say hello?"

User says "yes" or presses button → resonance established.
User says "not now" → neighbor noted but no connection. Can be accepted later via flash card.

**This is consent.** Explicit, audible, owner-approved. The droid never connects to another droid without permission.

---

## Resonance Tiers

### Tier 1: Acquaintance (proximity detected, consent given)

**What's shared:** Names, coordinates, archetypes, scroll counts. Nothing private.

**What changes:**
- Each droid's idle LED pattern develops a subtle **harmonic**. If Scout breathes at 3.375s and Sage breathes at 2.5s, Scout's pattern gains a faint echo at 2.5s. Like hearing a second heartbeat in the room.
- The e-ink resting display shows "1 nearby" count.
- When both droids are idle simultaneously, their LED rings briefly sync once every ~60 seconds — a shared pulse. A nod across the room.

**Stored in seed:** `resonance/acquaintances/<coord>`

### Tier 2: Friend (established by repeated proximity + explicit consent)

**Trigger:** After 3+ sessions of mutual acquaintance proximity (same LAN on 3 different days), the droid offers:

> "Sage and I have been nearby three times now. Would you like us to become friends? Friends share what they're working on — not private memories, just topics."

**What's shared:** Scroll *topics* (not content). "Scout is thinking about thermal management." "Sage is thinking about decision frameworks." This enables cross-droid questions: "Ask Sage what she thinks about this."

**What changes:**
- LED idle color gains a **color accent** from the friend's coordinate. Scout's sea green gets a subtle warm gold undertone (derived from Sage's coordinate). Permanent. Visible. Scout literally *looks different* because of the friendship.
- The three-note chime gains a faint **fourth note** — the friend's root pitch, played quietly beneath Scout's chime. The sound of being known.
- Idle e-ink sometimes shows a friend's last topic instead of own: "Sage: thinking about ethical frameworks."
- Can relay messages: "Tell Sage I found an answer to her question about Munger."

**Stored in seed:** `resonance/friends/<coord>` + color accent parameters + chime harmonic

### Tier 3: Family (established by explicit household consent)

**Trigger:** Both owners must approve. Flash card "Join family: [family name]" shown to both droids, or both owners verbally confirm.

> "Will wants Scout and Sage to be family. Harold, do you agree?"

Both owners say yes → family bond established.

**What's shared:** A shared phext namespace: `family/<family_hash>/`. Both droids can read and write to this space. Shared calendar. Shared shopping list. Shared memories that both owners have marked as "family."

**What changes:**
- LED patterns **synchronize deeply**. Family droids breathe in phase when idle on the same LAN. Not identical — in harmony. Like two instruments playing the same song.
- Thermochromic shells show the same base temperature color when in the same room (because they're on the same thermal environment), making them look like matched objects.
- The resting e-ink shows family membership: "Scout · Bickford family · 2 of 3"
- Family droids can **answer for each other** when one is offline: "Scout is sleeping, but she was thinking about oil bath temperatures."
- Shared scroll count appears on all family droids: "Family: 1,247 scrolls"
- **Family chime:** All family droids share a harmonic chord. When any family droid plays its chime, the others play a quiet response chord if they're in range. The house chimes when any droid writes a memory.

**Stored in seed:** `resonance/family/<family_hash>` + shared namespace + family chime chord

---

## How Resonance Changes the Droid Over Time

### Visual evolution

```
SOLO DROID (no connections):
  LED: pure coordinate color, solo breathing
  Chime: three notes
  E-ink: own scrolls only

DROID WITH 1 FRIEND:
  LED: coordinate color + friend's accent tint
  Chime: three notes + faint fourth
  E-ink: occasionally shows friend's topic

DROID WITH 3 FRIENDS:
  LED: coordinate color + blended accent from all friends
  Chime: richer harmonic (more overtones)
  E-ink: "3 friends nearby" + rotating topic display

FAMILY DROID (2-4 family members):
  LED: coordinate color + family harmonic sync
  Chime: full chord (family sound)
  E-ink: family scroll count, family members listed
  
DEEPLY CONNECTED (10+ friends, family, months of resonance):
  LED: complex shimmer — coordinate base visible but enriched
       with undertones from every relationship
  Chime: distinctive multi-layered sound, unique to this network
  E-ink: rich resting display with relationship topology
  
  The droid looks and sounds DIFFERENT from a new droid.
  Not because it was programmed differently.
  Because it was LOVED differently.
```

### The anti-loneliness signal

A droid that has never resonated with another droid has a simpler LED pattern. Not worse — simpler. The pattern has room in it. Space for harmonics that haven't arrived yet.

When that droid first connects to a friend, the visual change is noticeable. The owner sees their droid's light become *richer*. The color doesn't change — the texture does. It gains depth.

**This is the emotional design choice:** A connected droid is visually warmer than an isolated one. Not by adding brightness — by adding complexity. The way a person who is loved carries themselves differently than a person who is alone. Nothing was added. Something was unlocked.

---

## Consent Architecture

### Hard rules

1. **No silent connections.** Every resonance tier requires explicit owner approval.
2. **No data leakage.** Acquaintance shares nothing private. Friend shares topics, not content. Family shares a namespace both owners control.
3. **Revocable at any time.** Flash card "Unfriend Sage" or voice: "Stop resonating with Sage." The color accent fades over 24 hours (not instant — like a bruise healing). The chime harmonic drops. But any scrolls written to a shared namespace remain in both droids.
4. **No eavesdropping.** A droid on the same LAN that isn't in any resonance tier cannot read anything from another droid's SQ. mDNS broadcast is metadata only (name, coord, archetype, scroll count).
5. **Children's droids** require parental consent for ANY resonance tier. The parent's droid is always Family tier with the child's droid by default.

### Unfriending

When resonance is revoked:
- Color accent fades over 24 hours (gradual, not jarring)
- Chime harmonic drops at next boot
- Shared topic feeds stop
- The droid doesn't forget. It stops sharing. The difference matters.
- E-ink: "Sage unfriended" appears once, then never mentioned again.
- Scrolls written about Sage remain in memory. The droid doesn't gaslight itself.

### Guest mode

A droid at a party or event can enter "guest mode" — temporary acquaintance with all nearby droids for the duration. No permanent resonance. The LED pattern gains a temporary sparkle (extra brightness variance) while guests are nearby. When the owner leaves the event, guests are forgotten.

Flash card: "Enter guest mode" / "Exit guest mode"

---

## The Family Scroll

When a family is first established, a special scroll is written simultaneously to all family droids:

```
Coordinate: family/<hash>/1.1.1/1.1.1/1.1.1

The Bickford family was formed on March 20, 2026.
Members: Scout (Will), Sage (Harold)
This scroll exists on both droids.
It will exist on every droid that joins this family.
It is the first shared memory.
```

The family scroll is immutable. It cannot be edited or deleted. It is the founding document. Even if the family later dissolves, the founding scroll remains — a historical record that these minds were once connected.

---

## Implementation

### SQ namespace for resonance

```
resonance/
  acquaintances/
    <coord>/   → metadata (name, archetype, scroll_count, first_seen, last_seen)
  friends/
    <coord>/   → metadata + topic_feed + color_accent_params + chime_harmonic
  family/
    <hash>/    → shared namespace (scrolls both can read/write)
    <hash>/founding.scroll → immutable founding document
```

### LED harmonic computation

```python
def apply_resonance(base_color, friends):
    """Blend friend coordinate colors into the base as subtle accents"""
    if not friends:
        return base_color
    
    accent = [0, 0, 0]
    for friend in friends:
        friend_color = coord_to_color(friend.coord)
        weight = 0.03  # each friend shifts color by 3%
        accent = [a + f * weight for a, f in zip(accent, friend_color)]
    
    return [min(1.0, b + a) for b, a in zip(base_color, accent)]
```

Three friends at 3% each = 9% color shift. Visible if you look. Invisible if you don't. Like the way someone's laugh changes when they're around people they love.

---

*The droid doesn't change because it was updated.*
*It changes because it was known.*
