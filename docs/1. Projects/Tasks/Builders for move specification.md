Builder patterns

# MoveBuilder
- Init with absolute necessities
	- Special or normal
- Builder tools
	- add_input
		- Defaults to None if no input is provided
	- set_requirement
		- replaces the requirement, defaults to grounded
	- add_action
		- Blanket implementation to add an action
	- wait(frames, cancellable)
		- Add a fc
	- add_cost(Cost)
		- Will add the requirement and the drain action
	- add_hitbox(HitboxBuilder)
- Add a sanity check. A sane move has:
	- An animation
	- A nonzero duration

# HitboxBuilder
- Init with HitBuilder::projectile or HitBuilder::attached
- Sanity check. A sane hitbox has:
	- Some stun and damage
- Builder tools
	- set_damage/stun/knockback/pushback(on_hit, on_block)
		- Set that property on hit and on block, both required
	- frame_advantage(on_hit, on_block)
		- Will be evaluated into a stun prop
		- Calculated after the whole move is known, as waits after the attack have an impact
	- set_hits(amount)
		- Defaults to 1
	- set_model(model)
		- Spawn a model for the hitbox
	- set_speed(speed)
		- Move the hitbox at constant speed
	- set_fixed_height(attackheight)
		- Set a fixed attack height
		- Defaults to hitbox height calculation
	- set_lifetime(lifetime)
		- Defaults to 3 frames, or until owner lands or is hit

