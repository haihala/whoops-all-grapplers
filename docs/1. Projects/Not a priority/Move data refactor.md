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