#blocked by [[Move phase format rework]] and [[MoveSituation extention]]

- Split current system into three
	- Link detection and correction
	- Cancel detection and correction
	- Actual starting of the move
- Have an enum for what type of a move start is happening
	- When starting the move, you could use this enum to in toasts
	- The info of if a move was done raw, cancelled into or linked into is something that could go into `MoveSituation`