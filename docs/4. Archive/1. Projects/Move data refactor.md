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
1. [x] Make a move struct
2. [x] Rethink cancelling to be intuitive
3. [x] Make a move bank that holds the character's moves
4. [x] Remove animation bank
5. [x] Remove HitboxManager
6. [x] Integrate inputs parsing to move bank
7. [x] No more HitboxManager
8. [x] Change PlayerState
	1. [x] No more animation bank, active move state is now a part of player state
	3. [x] Dash is a move instead of being a unique state
9. [x] Mapping of spawn id to entity with components
	1. [x] Load similarly to inputs
	2. [x] Trigger an event to spawn with a ttl
	3. [x] Spawn and despawn accordingly
10. [x] Use new cancel system for jumps and movement

Bugs:
- [x] Sometimes button does nothing
- [x] Forward punch stops the user
- [x] Wrong box is spawned on command normal
- [x] Special isn't interpreted (nvm the duration between moves was just low)
- [x] Crouch ignores cancels