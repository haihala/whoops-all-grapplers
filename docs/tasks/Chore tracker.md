# Bugs
## Watchlist
- There is an input recording strangeness, parrots don't always work, probably an ordering question.

# Misc improvement ideas
- Pushback should not apply to projectiles
- Newtype wrapper for frames. 
	- Currently using usize, which means frames 99% of the time, but not always.
- Could you mirror player animation/model with shaders?
	- Boxes may be harder to draw?
- Disable parrots in prod
- Is there a more elegant way to encode concepts such as "charge move" and "meter cost"