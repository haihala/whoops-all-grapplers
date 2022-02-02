# Ticking
- [x] Get rid of the tick function
- [x] Make domain specific systems. Performance doesn't matter here
	- [x] One for walking / crouching / standing
	- [x] One for move activation
	- [x] One for size adjustment
	- [x] One for move progression
	- [x] One for stun / freefall recovery
- [x] Remove indirectness (events) from creating attacks
	- [x] If you have to leave events, make a fuse like mechanism that doesn't require manually consuming events

# Other misc concerns
- Currently doing a move while crouching will nudge the charcter up a bit, causing issues.
	- Caused by the size change in crouching system probably
	- Possibly solved by new physics
	- Fix in [[Fix execution order]]
