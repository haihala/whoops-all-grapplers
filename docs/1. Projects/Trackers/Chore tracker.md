# General
- Move code to a subdirectory to allow docs/intermediate assets

# Input parsing
- Maybe only store index in parser head
- Pass previous head as is, but prevent it from activating moves
	- Currently negative edge fails after a long time, as buttons are cleared for previous frame

# Waiting for upstream
- Can you use const sin for angles in jumps yet?
- Could you mirror player animation/model with shaders?
	- #bevy08

# Other
- Projectiles should stop if they aren't active aka clashing.
	- Currently fast enough projectiles can pass each other while clashing.