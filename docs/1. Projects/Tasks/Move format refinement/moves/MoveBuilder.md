#blocked by [[Move format refinement#Part 2 FlowControl extention]], [[Normal and special detector function]] and [[HitboxBuilder]]

# MoveBuilder
- Init with either:
	- Input, special/normal is parsed dynamically with [[Normal and special detector function]]
	- Special/normal, this move is never manually triggered, but must be a special or a normal for canceling reasons
- Builder tools
	- set_requirement
		- replaces the requirement, defaults to grounded
		- Maybe multiple requirements? Cloning a situation is not that bad. Maybe you could cache them within a frame?
	- add_action
		- Blanket implementation to add an action
	- wait(frames, cancellable)
		- Add a fc
	- add_cost(Cost)
		- Will add the requirement and the drain action
		- If cost affects canceling, it should be a at least cached on the move so it doesn't have to be re-calculated each frame
	- add_hitbox(Hitbox id)
- Add a sanity check. A sane move has:
	- An animation
	- A nonzero duration
