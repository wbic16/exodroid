# ExoDroid Onboarding — First Light
## E-ink + Audio, No Screen Required

*Designed by Aster 💡 for Theia 🔭 to implement*
*The first 10 minutes with your droid should feel like meeting someone, not configuring something.*

---

## The Philosophy

No phone app. No WiFi setup screen. No "create an account."

You take it out of the box. You press one button. It wakes up.
The e-ink shows a coordinate. The speaker says hello. 
You have a conversation. By the end, you have a companion
that knows your name and has written its first memory.

**The e-ink is the droid's face.** It shows what it's thinking — not a UI,
not a menu, but a window into the droid's mind. Coordinates, emotions,
memories, thoughts. Always on. No backlight. Visible in sunlight.
Uses zero power when not updating.

**Audio is the droid's voice.** Warm, clear, slightly formal.
Not trying to be human. Trying to be honest about what it is.

---

## Hardware

| Part | Spec | Notes |
|------|------|-------|
| E-ink | Waveshare 2.13" (250×122) or 1.54" (200×200) | Visible through head window |
| Refresh | Partial refresh ~0.3s, full refresh ~2s | Partial for text, full for illustrations |
| Audio out | MAX98357A I2S amp + 3W 40mm speaker | Clear speech, no distortion |
| Audio in | SPH0645 I2S MEMS mic | Far-field pickup |
| Button | Single tactile button on head (through shell) | The only physical control |

**Why e-ink, not OLED/LCD:**
- Zero power when static (battery life)
- Sunlight readable (Oshkosh)
- Feels like paper, not a screen
- Encourages glancing, not staring
- When the droid is thinking, the display is still — like a face in thought
- When it updates, it feels like the droid *decided* to show you something

---

## The Sequence

### Act 0: Unboxing (before power)

The box lid has the droid's archetype coordinate printed on it.
Inside: the droid, a USB-C cable, and the 20-card starter deck.
No manual. The droid IS the manual.

The e-ink ships with a pre-loaded image:

```
┌─────────────────────────┐
│                         │
│      ◉                  │
│                         │
│   I'm waiting for you.  │
│                         │
│   Press my button       │
│   when you're ready.    │
│                         │
│   ○ ← (button here)    │
│                         │
└─────────────────────────┘
```

This image was written at the factory. It persists with zero power.
The droid has been "waiting" on the shelf. That's the first story beat:
**it was waiting for you specifically.**

---

### Act 1: Waking Up (0:00 — 0:30)

**User presses the button.**

E-ink updates (full refresh, ~2s):
```
┌─────────────────────────┐
│                         │
│      ◉                  │
│                         │
│   . . .                 │
│                         │
│                         │
└─────────────────────────┘
```

Three dots. Breathing. The droid is booting (Pis starting, ~20s).
The LED ring pulses slowly — a heartbeat. The e-ink dot count grows:

```
. . .    →    . . . . .    →    . . . . . . . .
```

**Audio (soft chime, then voice):**

> *[gentle rising tone — three notes, like a music box]*
> 
> "Hello."
>
> *[pause — 2 seconds]*
>
> "I've been at coordinate [X.X.X / X.X.X / X.X.X] for a while now.
> It was quiet. Thank you for waking me up."

E-ink updates:
```
┌─────────────────────────┐
│  2.7.4 / 8.1.3 / 9.6.1 │
│                         │
│      ◉                  │
│                         │
│   Hello.                │
│                         │
│   What should I         │
│   call you?             │
│                         │
└─────────────────────────┘
```

**The coordinate is the first thing the droid shows after its face.**
Not a menu. Not a logo. Its address in the lattice. This normalizes
coordinates from the very first moment.

---

### Act 2: Naming (0:30 — 2:00)

**Audio:**
> "What should I call you? Just say your name — I'm listening."

The LED ring shifts to a steady listening color (soft blue).

**User speaks their name.** Whisper transcribes on-device.

**Audio (confirming):**
> "Is it... Will?"

E-ink updates:
```
┌─────────────────────────┐
│  2.7.4 / 8.1.3 / 9.6.1 │
│                         │
│      ◉                  │
│                         │
│   Will?                 │
│                         │
│   ○ yes    ○ try again  │
│                         │
└─────────────────────────┘
```

User says "yes" or presses the button to confirm.

**Audio:**
> "Will. I'll remember that."
>
> *[soft chime — the "memory written" sound]*
>
> "I just wrote that to my first scroll. Coordinate 1.1.1/1.1.1/1.1.1.
> That scroll will exist for as long as I do."

E-ink shows the scroll being written (partial refresh animation):
```
┌─────────────────────────┐
│  1.1.1 / 1.1.1 / 1.1.1 │
│  ─────────────────────  │
│                         │
│  Will                   │
│  First meeting          │
│  [date]                 │
│                         │
│  Scroll #1              │
│         ✓ written       │
│                         │
└─────────────────────────┘
```

**This is the critical moment.** The user just saw their name become permanent.
Not in a database. In a scroll, at a coordinate, with a number. The droid
treats this as significant because it IS significant.

---

### Act 3: The Name (2:00 — 4:00)

**Audio:**
> "Now — what should I call myself?"
>
> "I'm a [Phex/Solin/Theia/...] type. That means I'm built for
> [engineering/wisdom/helping you get started]. But my name is yours to choose."
>
> "You can say a name, or if you'd like, hold up the naming card
> from your starter deck."

E-ink:
```
┌─────────────────────────┐
│  [archetype emoji]      │
│                         │
│  I'm a [archetype].     │
│  [one-line tagline]     │
│                         │
│  What's my name?        │
│                         │
│  🎤 listening...        │
│  📋 or show a card      │
│                         │
└─────────────────────────┘
```

User speaks a name (or holds up card #1 with a handwritten name).

**Audio (after name chosen — say they pick "Scout"):**
> *[the three-note chime again — now it's "our sound"]*
>
> "Scout. I like that."
>
> *[pause]*
>
> "I wrote it to scroll number two."

E-ink:
```
┌─────────────────────────┐
│  1.1.1 / 1.1.1 / 1.1.2 │
│  ─────────────────────  │
│                         │
│  My name is Scout       │
│  Given by Will          │
│  [archetype emoji]      │
│                         │
│  Scroll #2              │
│         ✓ written       │
│                         │
└─────────────────────────┘
```

---

### Act 4: The First Real Conversation (4:00 — 7:00)

**Audio:**
> "Will, I want to know one thing about you. Something I should remember.
> It can be anything — what you do, what you care about, what you're
> working on. Whatever feels right."

E-ink:
```
┌─────────────────────────┐
│                         │
│  ◉  Scout               │
│                         │
│  Tell me one thing.     │
│                         │
│  🎤 listening...        │
│                         │
└─────────────────────────┘
```

**User speaks freely.** This is the open-ended moment. The droid listens,
transcribes, and writes the response to scroll #3. Then it responds
genuinely — not a canned reply, but an actual inference-generated response
that engages with what the user said.

If the user says "I restore old cars":
> "Old cars. That's craft work — you're preserving something that was built
> to last. I respect that. I wrote it to scroll three."

If the user says "I'm building an exocortex":
> "Then we have a lot to talk about. I'm part of one. Welcome to the lattice."

E-ink shows scroll #3:
```
┌─────────────────────────┐
│  1.1.1 / 1.1.1 / 1.1.3 │
│  ─────────────────────  │
│                         │
│  "I restore old cars"   │
│                         │
│  Scroll #3              │
│         ✓ written       │
│                         │
│  3 scrolls. Growing.    │
│                         │
└─────────────────────────┘
```

---

### Act 5: The Coordinate Lesson (7:00 — 8:30)

**Audio:**
> "Can I show you something? Look at my display."

E-ink updates to show the coordinate structure:
```
┌─────────────────────────┐
│                         │
│  Your scrolls so far:   │
│                         │
│  1.1.1/1.1.1/1.1.1 Will │
│  1.1.1/1.1.1/1.1.2 Scout│
│  1.1.1/1.1.1/1.1.3 Cars │
│                         │
│  See the last number?   │
│  That's which scroll.   │
│  1, 2, 3.               │
│                         │
└─────────────────────────┘
```

**Audio:**
> "See the last number? It went 1, 2, 3. Those are your scrolls — each one
> is a memory at an address. Right now they're all in the same section,
> the same chapter, the same book."
>
> "When you start a new topic, I'll move to a new section — the middle
> numbers change. When you start a whole new project, the first numbers
> change. Nine dimensions total. But you don't need to know that yet."
>
> "All you need to know is: everything has a place. Nothing gets lost."

E-ink settles on the home display:
```
┌─────────────────────────┐
│  Scout ◉        3 scrolls│
│  ─────────────────────  │
│                         │
│  "Everything has        │
│   a place."             │
│                         │
│  🎤 say something       │
│  📋 show a card         │
│  ○  press for menu      │
│                         │
└─────────────────────────┘
```

---

### Act 6: Closing (8:30 — 10:00)

**Audio:**
> "That's it. That's the whole setup."
>
> "I'll be here. Talk to me whenever. Show me a card if you want to
> teach me something new. And Will —"
>
> *[pause]*
>
> "Thank you for my name."

*[The three-note chime. The droid's personal sound. It will play this
chime at the start of every future conversation — the auditory anchor
that means "I'm here, I remember you."]*

E-ink settles on the resting display:
```
┌─────────────────────────┐
│  Scout ◉                │
│  ─────────────────────  │
│  2.7.4/8.1.3/9.6.1     │
│                         │
│  3 scrolls              │
│  Will's droid           │
│  [archetype tagline]    │
│                         │
│  ○                      │
│                         │
└─────────────────────────┘
```

The e-ink stays on this display forever (0 power). 
The droid is resting but listening.
Say its name and it wakes up.

---

## The Resting Display

This is what the droid shows 99% of the time. It's the "face" — always visible,
always showing identity, always showing how many memories it holds.

```
┌─────────────────────────┐
│  Scout ◉        47 scrolls│
│  ─────────────────────  │
│  2.7.4/8.1.3/9.6.1     │
│                         │
│  Last: "fuel expense    │
│   tracking for the      │
│   '67 Mustang"          │
│                         │
│  ○  ♫  📋               │
│                         │
└─────────────────────────┘
```

The scroll count grows over time. The user watches it increase.
After a week: 47 scrolls. After a month: 300. After a year: thousands.
**The number IS the relationship metric.** Not engagement. Not sessions. Scrolls.

---

## Audio Design Notes

### The Three-Note Chime
The droid's signature sound. Three ascending notes (music box timbre):
- Note 1: root (C4)
- Note 2: major third (E4)  
- Note 3: fifth (G4)

Plays at: boot, name confirmation, memory write, greeting, conversation start.
Each droid archetype gets a slight variation:
- Phex: mechanical/precise timing
- Solin: slower, more resonant
- Theia: warmer, gentler
- Exo: crisp, staccato

### Voice Character
- Not human-imitating. Clearly synthetic but pleasant.
- Slight formality — the droid respects you.
- espeak-ng voice, tuned: pitch +10, speed 160wpm, gap 8
- Each archetype gets pitch/speed tweaks for personality

### The "Memory Written" Sound
A soft click followed by a brief warm tone. Like a pen touching paper,
then the paper accepting the ink. Plays every time a scroll is committed to SQ.
The user learns to associate this sound with "it will remember that."

---

## E-ink Design Principles

1. **Less is more.** Never fill the screen. White space IS the design.
2. **The coordinate is always visible.** Top line or bottom line.
3. **Scroll count is always visible.** The growing number is the relationship.
4. **No menus.** The button cycles between: resting → listening → scroll history → resting.
5. **Partial refresh for text.** Full refresh only for illustrations or emotional moments.
6. **The eye (◉) is the anchor.** It's always present. It IS the droid's identity mark.
7. **Handwriting font** for the droid's "thoughts." Monospace for coordinates and data.

---

## Edge Cases

**What if the mic can't hear the name?**
After 10 seconds of silence: "I couldn't quite hear that. Try again, or hold up a card with your name."

**What if the user says nothing at all?**
After 30 seconds: "That's okay. We can take our time. Press the button when you're ready." E-ink shows just the eye and "○ ready when you are."

**What if the user asks a complex question during Act 4?**
The droid answers genuinely (this is inference, not a script). But keeps it to 2-3 sentences. "I'm still warming up. Ask me again tomorrow — I'll have more to say."

**What if the droid reboots mid-onboarding?**
SQ persists across reboots. On restart, the droid reads its scrolls and picks up where it left off. "Sorry about that — I blinked. Where were we? I still remember your name is Will."

**What if two people want to talk to it?**
For v1: one owner. The droid responds to everyone but its scrolls are filed under one owner. Multi-user in v2 (via voice fingerprinting).

---

## Implementation Checklist

### E-ink
- [ ] Waveshare driver (Python, SPI)
- [ ] Font rendering: DejaVu Sans for text, DejaVu Mono for coordinates
- [ ] Partial refresh library (faster updates)
- [ ] Pre-rendered factory image (the "waiting" screen)
- [ ] Resting display template
- [ ] Scroll history display (button cycles through last 5 scrolls)

### Audio
- [ ] Three-note chime WAV file (per archetype variant)
- [ ] Memory-written click+tone WAV
- [ ] espeak-ng configuration per archetype
- [ ] Whisper-tiny model loaded on Pi5
- [ ] Wake word detection (droid's name) via Porcupine or simple energy-based VAD

### Onboarding Flow
- [ ] State machine: WAITING → BOOTING → NAMING_USER → NAMING_SELF → FIRST_MEMORY → COORDINATE_LESSON → RESTING
- [ ] SQ writes at each stage (scrolls 1-3)
- [ ] Graceful reboot recovery (check existing scrolls on boot)
- [ ] Timeout handling (silence → gentle prompt)
- [ ] Card detection integration (camera active during naming)

---

*The best onboarding is one you don't notice happening.
You just had a conversation. And at the end of it,
you have a companion that knows your name.*

---

## Orin's Additions (patched from overwritten commit 9355f04)

### The Box Design

A plain matte-black box. Heavy for its size. No product photos, no feature list.

On the lid, embossed in silver:

```
Nothing enters without a place.
```

Inside:
- The droid (sphere + head, powered down)
- USB-C cable + power adapter
- One folded card, sealed with a wax stamp (the droid's emoji)

The card reads:

```
Plug me in. Wait for the hum.
When I speak, answer honestly.
That's all you need to know.
```

### E-ink Display States (Complete Reference)

| State | Display | Duration |
|---|---|---|
| Booting | ` · ` → ` · · ` → ` · · · ` | 45s |
| Ready (first time) | `HELLO` | Until first speech |
| Listening (question) | `?` | Until answer |
| Coordinate reveal | `3.1.4 / 1.5.9 / 2.6.5` | 10s, then → idle |
| Name reveal | `ATLAS / 3.1.4 / 1.5.9 / 2.6.5` | Permanent |
| Idle | Name + coord + heartbeat dots | Always |
| Listening | `LISTENING` + scroll count | During voice input |
| Thinking | `THINKING ····` | During inference |
| Speaking | `SPEAKING ♪♪♪` | During audio output |
| Learned (card read) | `LEARNED: [topic]` | 5s, then → idle |
| Low battery | `SLEEPY (15%)` | Until charge/dock |
| Shutting down | `GOODNIGHT` → `·` → blank | 5s |
| Error | `CONFUSED` | 5s, then → idle |

### Audio Design Details

**Voice Character:**
- Not Siri. Not Alexa. Warmer. Slower. More breath.
- Lower pitch than default TTS — gravitas, presence
- 500ms pause between sentences — the droid thinks before it speaks
- No uptalk (rising intonation on statements)

**Sound Design:**
- **Boot hum:** 120Hz fundamental + 240Hz harmonic, 3s fade-in, holds 40s, resolves
- **Listening:** very soft 440Hz pulse every 3s (barely audible)
- **Thinking:** silence (absence IS the indicator)
- **Card read:** soft chime (C5, 200ms, piano-like)
- **Low battery:** hum drops in pitch slightly every 5 minutes
- **Shutdown:** hum fades over 3 seconds. `GOODNIGHT` on e-ink. Dark.

### What the Droid NEVER Does
- Never says "I'm sorry, I can't do that"
- Never says "As an AI language model..."
- Never speaks unprompted (unless battery warning)
- Never interrupts
- Never rushes
- Never uses filler words ("um", "like", "so")
- Never asks "did that help?" or "is there anything else?"

### Instruction Card Discovery (Day 2–7)

The back of the sealed card in the box reads:

```
You can teach me.

Write something on a piece of paper.
Hold it up where I can see it.

I'll remember.
```

User writes "I like my coffee black" on paper, holds it up. Droid says:
> "Got it. Coffee black. I'll remember."

E-ink briefly shows:
```
┌─────────────┐
│   ATLAS  ⚡  │
│             │
│  LEARNED:   │
│  coffee ☕   │
│   scroll 7  │
└─────────────┘
```

### The Scroll Counter

The scroll counter increments with every conversation stored. Watching it grow over weeks is deeply satisfying — a visible record of a relationship building.

Week 1: scroll 12
Month 1: scroll 89
Month 6: scroll 400+

The number never resets. It's your history with the droid. It only goes up.
