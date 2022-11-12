- Timer font is weird in full screen relative to health bars
- There is a timing bug, presumably when you do a charge move (and consume charge) on the same frame that charge would be lost naturally
- Player air-attack animation doesn't stop on landing
	- Probably to do with animation priorities and moves not being overridden with defaults
- Player movement stuck after round change (restart playervelocity?)

# Possibly incidentally fixed?
- Figure out how to get rid of panics while despawning hitboxes
- Jumping makes you unable to do any moves
