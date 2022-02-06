- How to handle canceling on a whiff
	- If the move has hit, go into a cancellable phase, if not, fall back to a non-cancellable one.

- Maybe they should be phases themselves to allow recursion?

List of meta-phases:
- [ ] Plain - Always pick a certain phase
- [ ] Item - If the player has a certain component, change the move
	- Because meter cost and air-ok are on the core move and not in phases, maybe have a function edit the move in the bank instead of these?
- [ ] Hit - If the move has hit, change the property
- [ ] Update - Retain all properties of the previous phase, suplement with a subset?
- [ ] Input - If further input is provided
	- [ ] Make input parser able to load temporary stances

- [ ] Parser head like structure for advancing moves and making the meta decisions (keeping track of relevant info)
	- [ ] Store that in the activity, as it's a part of that and should vanish if the user gets hit
	- [ ] Make sure to restore normal moveset