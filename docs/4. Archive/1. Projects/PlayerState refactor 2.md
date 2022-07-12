# Crouching is a higher level state
- Currently there is only one state for the following:
	- Blocking
	- Move
- If a move is done crouching, this looks weird, same with low blocking

# Remove PlayerState and renaming PrimaryState
- Currently PlayerState only contains a PrimaryState and nothing else
- This is probably the case until the end of time