# Debugging
- [x] Attack hangs
- [x] Extra attack events
- [x] Spazzing walk state
- [x] Dashes are too long

# State
- [x] Combine to a mass enum to eliminate illegal state

# Physics
- [x] Detect collisions
- [x] Interface
	- [x] Add an impulse
	- [x] What is the current velocity
	- [x] Update current velocity based on state

# Basic ground movement
- [x] forward or back = run

# Basic air movement
- [x] Up = neutral jump
	- [x] Apply impulse

# Dashing
- [x] 656 or 454 to dash
- [x] Define dash in terms of travel time and distance like jumping
- [x] Dashing state (Uninterruptable start, interruptable follow up)
	- In this state, continuous movement is applied according to a function
		- Start fast, constant slide, drag to a stop
	- Dash start
		- Character is busy
		- Character is invulnerable
	- Dash end
		- Can jump and do attacks
		- Can't run or crawl
		- Can maybe duck
		- Can't block
