- How to handle canceling on a whiff
	- If the move has hit, go into a cancellable phase, if not, fall back to a non-cancellable one.

List of meta-phases:
- Plain - Always pick a certain phase
- Item - If the player has a certain component, change the move
	- Because meter cost and air-ok are on the core move and not in phases, maybe have a function edit the move in the bank instead of these?
- Hit - If the move has hit, change the property
- Input - If further input is provided

How to get the information to the move?
- Move advancer system, do this after [[Player state refactor]]