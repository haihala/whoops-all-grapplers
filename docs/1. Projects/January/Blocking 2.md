- [x] High/Low/Mid are tags on attacks
- [x] Detect block level and react accordingly
- [ ] Push back attacker on blocked hits

How to represent attack height
- Enum (high low mid)
- Numeric height on both hitboxes and blocker has thresholds
	- Maybe hitbox height is determined live
	- hitbox top < low threshold = low
	- hitbox bottom > high threshold = overhead
	- How to handle crouching
		- Player state takes size params in on startup
		- On hit ask if hit of this height can be blocked with this stick position 