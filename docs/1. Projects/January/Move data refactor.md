Blocked by: [[Input parsing v4]]

- Final move struct
	- Mobility (How the character moves during the move)
	- Spawns (Summons and projectiles)
	- Hitbox
	- Prerequisite status
	- Cost (Health and Meter)
	- Input
	- Follow ups (what can it cancel into)
	- Animation (frames and rigging)
	- On hit effect (status / damage / knockback)

- Barebones simplified version:
	- Input
	- Hitbox
	- Animation frames
	- Damage
	- Knockback

Steps
1. [ ] Make a move struct
2. [ ] Make a move bank that holds the character's moves
3. [ ] Simplify animation bank to only store current animation steps
4. [ ] Integrate inputs parsing to move bank


From taskboard:
# Define move data in phases, each phase has:

Startup, active, recovery is a three phase process, multihitting moves get more phases. 

A phase is an enum. One of:
-animation(recovery or startup)
-spawn(projectile)
-active

Every phase has:
-duration
-optional position at the end (if it moves you)
-freedom level

Spawn has a reference to the spawned object. (Maybe static mapping from id)

Active has:
-optional 'hit' (damage, knockback, stuff that will be relevant if it connects)
-optional hitbox(size and offset)

Dash is a move with two animation phases.

In addition to phases, moves have:
-input
-costs