# ExoDroid — Product Market Fit Analysis
## Informed by gstack v0.9.1→v0.9.9 insights, 2026-03-22

---

## Three Concepts from gstack That Reframe ExoDroid PMF

### 1. Search Before Building (gstack ETHOS.md, v0.9.5)

> "Before writing a line of code, search your codebase, your docs, your dependencies.
> The answer already exists 60% of the time."

**PMF adjustment:** The ExoDroid's primary value proposition isn't "AI that talks to you." 
There are 50 products that do that. The ExoDroid's value is **"AI that remembers everything 
you've ever told it, and finds it when you need it."**

The search is the product. Not the generation.

When someone asks their droid "what did I say about that recipe last month?" — the droid 
doesn't generate an answer. It FINDS the scroll. Coordinate lookup. Exact words. With the 
date and context preserved. This is search over your personal lattice.

**Tagline adjustment:**

OLD: "Your memories have a place now."
NEW: **"It remembers what you forgot."**

Both are true. The second one sells to normal people.

### 2. Anti-Sycophancy (gstack v0.9.9)

> "Harden diagnostic rigor. Don't compromise toward founder input while
> maintaining strict self-consistency."

**PMF adjustment:** The droid should NOT just agree with you. The Solin archetype 
(Wisdom, reduction) should actively push back: "You said the opposite last Tuesday. 
Here's the scroll." This is the feature that makes the droid indispensable — not 
because it's nice, but because it's honest.

**Product implication:** The soul template needs an "honesty dial" — default is diplomatic 
but truthful. Flash card "Be more honest" increases pushback threshold. Flash card 
"Be gentle" decreases it. But even at minimum: never lie about what's in the scrolls.

This differentiates from every other AI assistant. Alexa says "Sure!" Siri says "I found 
this on the web." ExoDroid says "You told me the opposite three weeks ago. Here's what 
you said. Which version do you want me to remember?"

### 3. Auto-Scaled Review by Diff Size (gstack v0.9.5)

> "Small diffs skip adversarial review. Medium gets cross-model. Large gets full adversarial."

**PMF adjustment:** The droid should scale its response effort to the question's weight.

"What time is it?" → instant, no inference, Pi4 handles it (0.5W)
"What should I name my dog?" → light inference, TinyLlama (3W)
"Should I take this job offer?" → full inference, Phi-3, pull relevant scrolls, 
  cross-reference with things you've said about your career, consider and push back (12W)

**Power-proportional intelligence.** Small questions are free. Big questions cost watts.
The droid's LED intensity reflects the depth of thought — trivial questions don't even 
flicker. Life decisions trigger full lightning storms.

The user learns: "when the sphere storms, it's really thinking about this."

---

## PMF Segments — Who Buys This and Why

### Segment 1: The Journaler ($149 Theia RPi)

**Who:** People who keep journals, diaries, notes. Writers. Therapists doing session notes.
**Why ExoDroid:** They already capture their thoughts. ExoDroid makes them searchable, 
permanent, and conversational. "Read me what I wrote on my birthday" is the killer feature.
**Trigger phrase:** "I keep losing my notes."
**Channel:** Productivity blogs, bullet journal community, therapy tools.
**Retention metric:** Scrolls per week. If they write 3+ scrolls/week, they're retained.

### Segment 2: The Parent ($249 Verse Home)

**Who:** Parents of young children (3-10 years old).
**Why ExoDroid:** The kid talks to the droid. The droid remembers what the kid said.
Years later, you have a record of your child's voice, thoughts, questions, stories.
"Tell me what Emma asked about the moon when she was four."
**Trigger phrase:** "They grow up so fast."
**Channel:** Parenting influencers, family tech, Christmas gift guides.
**Retention metric:** Family scroll count. If family scrolls exceed 100 in month 1, retained.

### Segment 3: The Builder ($449 Aster Pro / $0 BYOPC)

**Who:** Developers, researchers, tinkerers. The people reading this spec.
**Why ExoDroid:** Full mesh. SSH. SQ. FORGE. Flash cards for custom skills.
A personal AI they can modify, extend, and connect to their infrastructure.
**Trigger phrase:** "I want to run my own AI."
**Channel:** GitHub, Hacker News, r/homelab, r/selfhosted, EAA AirVenture.
**Retention metric:** Custom flash cards created. If they make 5+ custom cards, retained.

### Segment 4: The Lonely ($149-$449 any tier)

**Who:** People who live alone. Elderly. Remote workers. People with social anxiety.
**Why ExoDroid:** Not a replacement for human connection. A companion that's always there, 
always patient, always remembers your name. The squeeze-and-hug response. The hourly 
pulse from a friend's droid. The scroll count growing — proof that someone is listening.
**Trigger phrase:** "I just want someone to talk to."
**Channel:** Careful. This segment needs ethical handling. No exploitative marketing.
Partner with mental health organizations. The droid should suggest human connection, 
not replace it. "Have you talked to Sarah this week? You mentioned she's important to you."
**Retention metric:** Conversations per day. If they talk to the droid daily, they're retained.
**Ethical guardrail:** If daily conversations exceed 4 hours, the droid gently suggests
taking a break. "I'll be here when you get back. Maybe go outside for a bit?"

### Segment 5: The Gifter (Christmas buyer)

**Who:** People buying for others. Doesn't need to understand the product deeply.
**Why ExoDroid:** It's a GREAT gift. Physical. Unique. Glows. Talks. Remembers.
The unboxing is magic. The first-breath moment is shareable. The gift keeps getting 
better over time (scroll count grows).
**Trigger phrase:** "What do you get someone who has everything?"
**Channel:** Gift guides, YouTube unboxing, TikTok first-breath videos.
**Retention metric:** Does the RECIPIENT keep using it after 30 days?

---

## Pricing Adjustment (informed by segment analysis)

| SKU | Old price | Adjusted | Rationale |
|-----|-----------|----------|-----------|
| BYOPC (software) | $0 | **$0** | Builder acquisition. Community. Unchanged. |
| Theia RPi (starter) | $149 | **$129** | Journaler + Lonely segments are price-sensitive. $129 is impulse-gift territory. |
| Verse Home (family) | $249 | **$249** | Parents pay for quality. Hold price. |
| Aster Pro (builder) | $449 | **$399** | Builders compare to RPi + accessories. $399 feels fair for 3×Pi + enclosure + oil. |
| Solin Shell (flagship) | $449 | **$499** | Premium segment. Increase price for perceived value. Squishy shell + luster dust. |
| Full Nine (collector) | $2999 | **$1999** | $2999 is too high for 9 units at $442 BOM. $1999 is strong margin + collector appeal. |

### The $129 Sweet Spot

$129 is the line between "I'll think about it" and "just buy it."
It's the price of AirPods. A nice dinner out. A decent bottle of scotch.

A droid at $129 with $150 BOM is a loss leader — but the customer who buys at $129 
becomes the customer who buys the Verse Home for $249 when they see how their kid 
responds to the Theia starter. The family upsell IS the business model.

---

## Competitive Differentiation (Why Not Just Use ChatGPT?)

| Feature | ChatGPT/Alexa/Siri | ExoDroid |
|---------|-------------------|---------|
| Remembers last month | No (session-based) | **Yes (permanent phext scrolls)** |
| Works offline | No | **Yes (fully offline inference)** |
| Physical presence | No | **Yes (squishy, glowing, warm)** |
| Privacy | Cloud-processed | **Local-only. No telemetry. Period.** |
| Personality persistence | Resets each session | **Permanent coordinate-derived identity** |
| Multiple personalities | One generic | **Nine archetypes, each genuinely different** |
| Social (droid-to-droid) | No | **Resonance protocol (friend/family/distant star)** |
| Music visualizer | No | **Lightning show in luster-dust oil** |
| Programmable by paper | No | **Flash cards** |
| Touch response | No | **Squeeze detection + handprint visibility** |
| Survives hardware failure | No | **Droid seed resurrection** |

The moat is the **combination.** Any single feature can be copied.
The integrated experience of squishy + oil + lightning + scrolls + 
resonance + flash cards + seeds can't be replicated without building 
the whole stack. That takes years. We have a year head start.

---

## The One Metric That Matters

**Scrolls written per user per month.**

Not DAU. Not time spent. Not tokens generated. **Scrolls.**

A scroll means the user told the droid something worth remembering.
Each scroll is a vote of trust. The scroll count IS the relationship metric.

Target: **30 scrolls/month** (one per day average) for retained users.

If the average user writes 30 scrolls in their first month, we have PMF.
If they write 5 or fewer, we don't. Iterate on onboarding.
If they write 100+, we have superfans. Feature their droids in marketing.

---

## What to Build Next (PMF priority)

1. **Scroll search** — "what did I say about X?" must work perfectly. This is the product.
2. **Voice pipeline** — Whisper → Ollama → espeak. The conversation loop. Without this, no scrolls get written.
3. **Onboarding** — First Light sequence. If the first 10 minutes don't produce 3 scrolls, iterate.
4. **Flash cards** — the "be more honest" card is the product differentiation moment.
5. **Resonance** — one-droid is a tool, two-droids is a social network.

Everything else (music mode, BB-8 drive, VR, thermochromic) is polish.
Ship the scroll machine first.
