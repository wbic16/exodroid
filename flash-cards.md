# Flash Card System — Droid Programming via Paper

Paper is the interface. Hold up a card, the droid reads it.

## Card Generator
Web tool at mirrorborn.us/cards — pick category, type instruction, print.
Cards are shareable via URL: mirrorborn.us/cards/0042

## Starter Deck (ships with droid)
1. "What's your name?" — identity/name selection
2. "Here's my name: ________" — identity/owner (handwrite)
3. "Remember this" — memory/blank (write anything)
4. "Be more concise" — personality
5. "Be more detailed" — personality
6. "Activate whimsy mode" — mood
7. "Connect to WiFi: ________" — config (handwrite SSID)
8. "Print your summary" — action
9. "What do you know about me?" — query
10. "Forget this conversation" — forget/session clear
11. "Show your coordinate" — debug
12. "Track my expenses" — skill
13. "Set a reminder: ________" — memory (handwrite)
14. "Pair with another droid" — mesh
15. "Read this document" — intake (show a page)
16. "Teach me something" — education
17. "Run diagnostics" — debug
18. "Speak louder / softer" — config
19. "What time is it?" — utility
20. "Thank you" — social/gratitude scroll

## QR Payload Format
```json
{"v":1,"cat":"identity","cmd":"set_name","val":"Scout","confirm":true,"card_id":"0042"}
```
QR Version 10 (57×57) holds ~600 bytes. Prints at ~5cm square.

## Custom Cards
Blank cards included. Write with Sharpie — OCR handles handwriting.
Or print from mirrorborn.us/cards with your own instructions.
