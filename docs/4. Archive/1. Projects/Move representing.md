Types of moves the current system can't really represent

- Multi parters (Rekka)
- Hold phases (Vortex)
- Repeated phases (Multiple gunshots released with a single draw)
- Actions that don't follow the phase structure (Parry with Gi)
	- This could be left as a special case

Required mechanisms
- Further input mid-move
	- Add flags to phasecondition and instead of a phase, it either returns a phase or a move id (start a new move)
	- Input is optional (moves without input cannot be accessed directly)
- New flags for when an input is given
	- System to set input flags
	- For now just set to current input state, only check on phase change

Recheck of requirements:
- Multi parters - Easy, just add a MoveAction to start a new phase
- Holders - Fine, just make the holdable part a recursive move
	- On second thought, this will cause the game to go into infinite recursion trying to calculate move length
	- Remove method for total length, moveaction returns an option, None if it's a new move
- Repetitions - Fine, just multi-phase moves, work similarly to hold
