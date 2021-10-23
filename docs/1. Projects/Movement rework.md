# TODO
Collision seems a tad wonky with statics
- Could be a floating point thing

Crouching

Crawling

Think about re-introducing some weight to the movement

# Physics
- [x] Impulse (velocity change)
	- Fire and forget, these cannot be accessed afterwards
- [x] Detect collisions
- [x] Interface
	- [x] Add an impulse
	- [x] Get current velocity

# Basic ground movement
- [x] forward or back = run
- [ ] directly down = ducking (used to dodge stuff)
	- [ ] Shrink visuals
	- [ ] Shrink hitbox
- [ ] down forward or back = crawl
	- [ ] Shrink visuals (but less than ducking)
	- [ ] Shrink hitbox (but less than ducking)
	- [ ] Apply continuous force

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
