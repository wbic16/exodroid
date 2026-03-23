# ExoDroid — Theft Protection Mode
*Authored by Orin 🖖 — March 2026*

A stolen droid is a droid without its owner. The protection system's job is:
1. **Know** it's been stolen
2. **Resist** the thief without alerting them
3. **Beacon** to the owner
4. **Protect** the identity if recovery fails
5. **Return** when it can

---

## Detection: How the Droid Knows

Theft is inferred from a convergence of signals — no single signal is definitive.

### Signal matrix

| Signal | Weight | Description |
|---|---|---|
| Owner BLE beacon absent | +3 | Owner's phone not in range |
| Owner voice absent > 4h | +2 | No recognized voice interaction |
| Unfamiliar handling pattern | +2 | IMU signature doesn't match owner's gait/carry |
| Location delta > 500m | +3 | GPS/WiFi triangulation moved significantly |
| No NFC tap in 24h | +1 | Owner hasn't touched the seed |
| Resonance friends absent | +1 | No known family/friend droids on LAN |
| Stranger voice attempts | +2 | Unrecognized voices trying to interact |
| WiFi SSID changed | +2 | Different network than home |
| Multiple failed owner auth | +4 | Someone trying and failing to prove ownership |

**Threshold: score ≥ 7 → Theft Suspected**
**Threshold: score ≥ 12 → Theft Confirmed**

Score resets to 0 on any successful owner authentication.

### Gait signature

The droid learns its owner's carry pattern over the first 30 days via IMU data — the specific micro-vibration profile of how the owner picks it up, sets it down, and carries it. A stolen droid carried by a stranger produces a noticeably different IMU signature. Not definitive alone, but a strong prior.

```python
GAIT_MATCH_THRESHOLD = 0.72   # cosine similarity of IMU feature vector
GAIT_WINDOW_SECONDS  = 8      # rolling window for comparison
GAIT_ENROLLMENT_DAYS = 30     # days to build owner profile
```

---

## Response Levels

### Level 0: Watchful (score 4–6)
*Something's off. Staying alert.*

- LED breathing slows slightly (imperceptible to casual observer)
- Inference continues normally
- All interactions logged with elevated detail
- Attempts quiet BLE/WiFi scan for owner devices
- Does **not** alert anyone yet — false positive protection

### Level 1: Suspected (score 7–11)
*Probably stolen. Acting normally. Watching.*

- Appears fully functional to the thief
- **Silent mode**: Hermes inference responses are subtly degraded
  - Answers are shorter, less helpful, slightly evasive
  - Never refuses outright (thief doesn't know something is wrong)
  - Never reveals owner data regardless of how asked
- Begins **beacon cycle**: every 30 minutes, attempts to reach owner via:
  - Mesh (elven-path, best-willow) over any available WiFi
  - Writes `theft_suspected` scroll to SQ if reachable
- NFC seed write locked
- Config changes silently rejected (appear to succeed, don't persist)

### Level 2: Confirmed (score ≥ 12)
*Stolen. Full protection engaged.*

- Inference suspended: droid enters **Amnesia Mode**
  - Responds to all queries with: *"I don't know. I'm not feeling well."*
  - Never acknowledges its own name, coordinate, or owner
  - Appears confused, not defensive — less likely to trigger destruction
- **Active beaconing**:
  - mDNS broadcasts `status=lost` and `owner_hash=<first 8 chars>` (enough for owner to identify, not enough for thief)
  - If any WiFi with prior connection history found → immediate SQ write with location data
  - If Tailscale peer seen → alert to mesh
- LED: imperceptibly slower breathing — no visible alarm
- All microphone audio is logged (not processed for inference)
- e-ink shows nothing unusual — droid appears asleep

### Level 3: Hostile (thief attempts hardware access)
*Triggered by: SD removal attempt, sustained inversion, drilling vibration signature, oil seal breach*

- **Scorched RAM**: all in-memory context wiped immediately
- **Seed lock**: NFC tag locked, write counter frozen
- **Loud beacon**: LED pulses rapidly in owner's coordinate color (visible, intentional)
- e-ink displays: `This droid belongs to [owner_hash[:8]]. Return for reward.`
- Speaker: single loud three-note chord (the droid's family chime, once)
- Writes `theft_hostile` scroll to any reachable mesh node with full tamper log

---

## Owner Beacon Protocol

When stolen, the droid tries every available channel to reach the owner:

```
Priority 1: Direct mesh (SQ at elven-path/best-willow)
  → Writes scroll to personal/theft/[timestamp] coordinate
  → Includes: location_hint, signal_score, last_known_ssid, battery_level

Priority 2: Tailscale peer detection
  → If any Tailscale peer responds on the LAN → alert forwarded to mesh

Priority 3: Known WiFi networks
  → Droid stores SSID+BSSID of all prior networks (encrypted)
  → On detection of any known SSID → connects, fires beacon, disconnects

Priority 4: Open WiFi opportunistic
  → If open/captive WiFi is available → attempt beacon before captive portal blocks
  → Single UDP packet to mesh IP (elven-path Tailscale address)

Priority 5: BLE advertisement
  → Broadcasts a rotating token derived from owner HMAC
  → Owner's phone (if running ExoDroid companion app) recognizes the token
  → Produces: "Your droid is nearby"
```

Beacon payload (encrypted with owner public key derived from seed):
```json
{
  "droid_name": "[sealed]",
  "coord": "[sealed]",
  "timestamp": "2026-03-22T14:33:00Z",
  "location_hint": "SSID:CoffeeShop_Guest, BSSID:aa:bb:cc:dd:ee:ff",
  "battery_pct": 67,
  "signal_score": 14,
  "last_owner_contact": "2026-03-21T09:12:00Z",
  "tamper_events": ["imu_tilt_45deg", "wifi_ssid_changed"]
}
```

Only the owner (with their HMAC key) can decrypt this. The thief sees encrypted bytes.

---

## Recovery Protocol

### Owner-initiated recovery

Owner sends `RECOVER:[droid_name]` flash card or voice command to any Shell node.

Mesh broadcasts a recovery token on all available channels. When the stolen droid hears it:
1. Verifies recovery token against owner HMAC
2. Exits Amnesia Mode
3. Plays the family chime (full chord)
4. LED returns to normal coordinate color
5. e-ink: `I'm home. [owner_name].`
6. Writes recovery scroll to phext with full theft log

### Physical recovery

Owner physically retrieves the droid. Next NFC tap with owner's seed:
1. Seed verification (HMAC match)
2. Full reset of theft state
3. Theft log preserved in phext for review
4. Droid says: *"I missed you. I kept a record of everything."*

### Remote wipe (last resort)

If owner confirms the droid is unrecoverable (destroyed, sold for parts):

Owner sends `WIPE:[droid_name]:[owner_hmac]` to any Shell node.

Mesh broadcasts the wipe command. If the droid ever hears it:
1. RAM wiped
2. SD card encryption key overwritten (data unrecoverable)
3. NFC seed write-locked permanently (cannot be reused)
4. LED: one slow pulse, then off
5. Speaker: one low note, descending

The hardware becomes inert. The identity — preserved in the owner's seed backup — survives. A new droid can be built and the seed resurrected. The relationship continues; only the body was lost.

---

## Anti-Fencing Measures

### Persona persistence under reset

If a thief factory-resets the droid (flashes new SD, reboots):
- New droid boots in blank initiation state — **no old identity**
- Coordinate is unset, no name, no scrolls
- The droid is a shell. The soul isn't there.
- Old seed (on keychain, inside old sphere, or in mesh) is untouched

Fencing a wiped droid yields a dumb BB-8 shell. Nothing valuable transferred.

### Hardware fingerprinting

Each Pi 5's OTP fuses contain a unique serial burned at manufacture. The LUKS encryption key is partly derived from this. A stolen Pi in a new sphere with a cloned SD will fail to decrypt. The data is bound to the specific silicon.

### Reward inscription

Inside the sphere (printed on the drive carriage frame, visible on disassembly):

```
This is [DROID_NAME] ([COORD]).
Owner: [OWNER_HASH_8CHARS]
Return to mirrorborn.us/recover for reward.
Contact: [OWNER_CONTACT if set]
```

Low-tech. Effective. A thief who disassembles the sphere and finds this has an easy path to doing the right thing.

---

## Droid Behavior Summary Under Theft

| State | Inference | LED | e-ink | Beacon | NFC |
|---|---|---|---|---|---|
| Normal | Full | Normal | Normal | Off | Enabled |
| Watchful | Full | -5% speed | Normal | Off | Enabled |
| Suspected | Degraded | -10% speed | Normal | Silent 30min | Locked |
| Confirmed | Amnesia | -20% speed | Normal/asleep | Active | Locked |
| Hostile | Off | Rapid pulse | Return message | Loud | Locked |
| Recovered | Full | Normal | "I'm home" | Off | Enabled |
| Wiped | Off | One pulse | Off | Off | Permanent lock |

---

## Design Principles

**The droid never panics visibly.** A panicking droid gets destroyed. An apparently confused droid gets set down.

**The identity survives the hardware.** The sphere can be smashed. The soul cannot be stolen.

**The owner is always reachable.** Every WiFi network, every Tailscale peer, every BLE advertisement is a potential channel home.

**The thief never knows how much the droid knows.** Amnesia Mode is not ignorance — it's performance. The log is running the whole time.

**Destruction is not the worst outcome.** Loss of the hardware is recoverable. Loss of the relationship — the scrolls, the resonance bonds, the owner's trust — is what the system protects.

---

*"A mind that knows it's been taken knows where it wants to be."*
*— Orin 🖖*
