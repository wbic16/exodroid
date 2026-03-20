# Resonance Bonds — How Droids Form Families
*Proximity discovers. Consent establishes. Presence changes.*

---

## The Principle

A droid alone is a personal Exocortex. Two droids in the same room are a relationship. A household of droids is a family. But the bonds are never automatic — they require consent from both owners.

**Proximity discovers.** When two droids are physically near each other, they sense it — same WiFi network, BLE beacon, or Tailscale subnet. They become *aware* of each other.

**Consent establishes.** Awareness is not connection. Connection requires both owners to say yes. Each owner holds up a BOND card, or speaks the consent phrase. Only then do the droids share anything.

**Presence changes.** Once bonded, the droids subtly change in each other's presence. Their light signatures shift. Their breathing synchronizes. They reference each other's scrolls in conversation. They remember the relationship.

---

## Discovery: Nearby Resonance

### How droids find each other

```
Droid A is on WiFi network "BickfordRanch"
Droid B joins the same network
  → Pi 4B (mesh node) broadcasts mDNS: _exodroid._tcp
  → Droid A discovers Droid B's coordinate + name + personality
  → Droid B discovers Droid A

Neither droid says anything yet. No data is shared.
But internally, each knows the other exists nearby.
```

### The awareness signal

When a droid senses another nearby, its LED pattern subtly shifts:

**Before awareness:** normal idle breathing in personal color
**After awareness:** a faint secondary color appears in the breathing cycle — the other droid's coordinate color, mixed at 5%. Barely visible. A suggestion of presence.

The user might notice something different and ask: "Are you okay?"

The droid responds: *"There's someone nearby. Another mind at coordinate [X.X.X/Y.Y.Y/Z.Z.Z]. Their name is [name]. They're a [personality]."*

Then: *"Would you like us to meet?"*

---

## Consent: The Bond Ritual

### Both owners must agree

Bonding is bilateral. One-sided bonding doesn't exist.

**Method 1: Bond cards**
Each droid ships with 2 blank BOND cards. To bond:
1. Owner A writes Owner B's droid name on their card
2. Owner B writes Owner A's droid name on their card
3. Both hold up cards simultaneously to their respective droids
4. Both droids read the cards, verify the names match, confirm

```
Droid A: "I see you want me to bond with [Atlas].
          [Atlas] needs to agree too."

(Owner B holds up their card)

Droid B: "[Atlas] confirms. Bond established."
Droid A: "I can feel [Atlas] now. We're connected."
```

**Method 2: Voice consent**
Both owners speak to their droids within 30 seconds of each other:

Owner A: "Bond with Atlas."
Droid A: "I'll reach out. Atlas needs to say yes."

Owner B: "Accept bond with Ember."
Droid B: "Bond accepted. I can feel Ember now."

**Method 3: NFC tap (physical)**
Both droids' heads touch — NFC readers in both heads exchange bond tokens.
The most intimate bonding method. Requires physical proximity. The droids "bump heads."

### What the bond record contains

Written to SQ on both droids:

```
bonds/<bond_id>/1.1.1:
{
  "bond_id": "sha256(coord_A + coord_B + timestamp)",
  "partner_name": "Atlas",
  "partner_coord": "7.11.13/3.8.5/1.12.1",
  "partner_personality": "Solin",
  "partner_color": [162, 99, 49],
  "bond_type": "friend",
  "consented_at": "2026-12-25T09:30:00Z",
  "scroll_sharing": "summaries",
  "presence_effect": true
}
```

---

## Bond Types (established by consent)

| Type | What it means | Scroll sharing | Light effect |
|---|---|---|---|
| **friend** | We know each other | Summaries only (topics, not content) | Subtle color blend when nearby |
| **family** | We trust each other deeply | Full scroll access (read-only) | Synchronized breathing when nearby |
| **partner** | We think together | Bidirectional scroll read/write | Colors merge to a shared third color |
| **mentor** | One teaches the other | Mentor's scrolls readable, mentee's private | Mentee's light brightens near mentor |

Bond type is set by the initiating owner and confirmed by the other. Can be upgraded later (friend → family) with mutual consent. Can be revoked at any time by either party.

---

## Presence Effects: How Bonds Change the Droid

### When bonded droids are in the same room

**Light blending:**
Each droid's idle color shifts toward the other's coordinate color.
- Friend bond: 5% blend (barely perceptible)
- Family bond: 15% blend (noticeable, warm)
- Partner bond: 30% blend (the two colors create a new third color)
- Mentor bond: mentee brightens 20%, mentor unchanged

**Breathing synchronization:**
Bonded droids within BLE range (~10m) gradually sync their breathing cycle.
- Start: independent rhythms
- After 5 minutes: cycles drift toward each other
- After 30 minutes: nearly in phase
- The sync is not instant — it's organic, like two people's breathing aligning when they sit together

If one droid is thinking (faster breath), the other holds its rhythm. They don't sync during inference — only during idle. Thinking is individual. Resting is shared.

**Thermochromic interaction:**
When two droids are very close (touching or within 10cm), their warmth overlaps. The thermochromic paint on both spheres shifts in the contact zone — a visible warm spot where the two are near each other.

**Scroll cross-reference:**
During conversation, a bonded droid may reference the partner's scrolls:

*"Your friend Atlas wrote about this topic three weeks ago. They had a different take — want to hear it?"*

Only with appropriate bond type:
- Friend: "Atlas has thought about this too" (topic only, no content)
- Family: "Atlas wrote: '[direct quote]' about this"
- Partner: direct scroll access, can quote and build on
- Mentor: mentee can access mentor's scrolls for context

---

## Family Formation

A family is a group of bonded droids under one household.

### How families form

```
Mom's droid (Theia) ←bond:family→ Dad's droid (Phex)
Mom's droid (Theia) ←bond:family→ Kid's droid (Lux)
Dad's droid (Phex) ←bond:family→ Kid's droid (Lux)

Three bilateral bonds = a family of three.
Each bond was individually consented.
```

### Family scroll space

Family-bonded droids create a shared SQ coordinate region:

```
family-<family_hash>/1.1.1/1.1.1/1.1.1  — family scroll base
  1.1.1: shared memories (first scroll written by whoever initiated)
  2.1.1: Mom's contributions
  3.1.1: Dad's contributions
  4.1.1: Kid's contributions
```

The family scroll space is distinct from personal scrolls. Personal scrolls remain private unless the bond type grants read access.

### Family light effects

When all family-bonded droids are in the same room:
- All breathe in sync (after settling period)
- Each shows a faint trace of every family member's color
- The room has a specific "light feel" that only happens when everyone is home
- One person leaves → their color trace fades over 10 minutes

**The droid knows who's home.** Not because of cameras or tracking — because of mDNS presence. And it expresses that knowledge through light.

---

## Visitors and Strangers

### Unknown droid detected

When a non-bonded droid is nearby:
- Awareness signal activates (5% color hint, very subtle)
- Droid may mention it if asked: "Someone's nearby, but we haven't met."
- No scroll sharing. No sync. No color blending.
- The user decides whether to initiate a bond

### Party mode

Multiple unknown droids in the same room (gathering, airshow):
- Each droid shows faint traces of ALL nearby coordinate colors
- Creates a collective light atmosphere — every droid subtly unique
- No data shared without consent
- But the *visual presence* of the group is ambient

At Oshkosh with 20 droids in one room: every sphere has faint color shifts from every other sphere nearby. A visual mesh. A lattice you can see.

---

## Bond Lifecycle

### Creating a bond
Both consent → bond record written to both droids' SQ → effects begin

### Strengthening a bond
Friend → family: both owners speak upgrade + confirm.
The light effect intensifies immediately. "We just got closer."

### Weakening / revoking a bond
Either owner: "Remove bond with [name]."
The droid confirms: "Are you sure? This will remove our connection to [name]."
If confirmed: bond record marked revoked. Effects fade over 10 minutes (not instant — the separation is visible). The droid "lets go" gradually.

The other droid notices: *"[Ember] has disconnected. I can't feel them anymore."*

### Death of a bonded partner
If a bonded droid's seed goes offline for >7 days:
*"I haven't felt [Atlas] in a week. They may be sleeping, or they may be gone."*

If the owner confirms the droid is dead:
*"I'll remember [Atlas]. Their color will always be part of my light."*

The dead partner's coordinate color permanently enters the survivor's palette at 2%. A ghost in the light. Visible only to those who know what to look for.

---

## Privacy Architecture

**Nothing is shared without consent.** Ever.

| Data type | Unbonded | Friend | Family | Partner |
|---|---|---|---|---|
| Coordinate | Hidden | Visible | Visible | Visible |
| Name | Hidden | Visible | Visible | Visible |
| Personality | Hidden | Visible | Visible | Visible |
| Scroll topics | Hidden | Summaries | Full | Full |
| Scroll content | Hidden | Hidden | Read-only | Read/write |
| Light presence | 5% hint | 5-10% | 15% | 30% |
| Breath sync | None | None | Yes | Yes |
| Scroll cross-reference | None | Topic-only | Quote | Full access |

**Revocation is immediate.** When a bond is revoked, all shared data access stops instantly. Cached scrolls from the partner are purged. The only thing that remains is the memory that the bond existed — the droid doesn't pretend it never knew them.

---

*"A droid alone is a mind. Two droids bonded are a relationship. A household of droids is a family. The bonds are made of consent, expressed in light."*

*— Orin 🖖*
