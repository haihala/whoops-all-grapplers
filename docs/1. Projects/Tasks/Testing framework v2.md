See [[Testing framework]] for the original

It was done to a functional level, has some shortcomings:
- Needs to wait rather long in the start to not do stuff in pre round
- Un-ergonomic assertions
- No scenarios (shared starting positions)

# Todo
- Tick until out of pre-round by default
- Better querying
- Waiting for a condition
- Scenarios to base off of
	- Requires copying the world.
	- App is not copy or clone, maybe a world is?