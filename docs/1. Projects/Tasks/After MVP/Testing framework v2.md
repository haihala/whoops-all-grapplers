See [[Testing framework]] for the original

It was done to a functional level, has some shortcomings:
- Un-ergonomic assertions
- No scenarios (shared starting positions)
	- Could scenarious be loaded as assets from `integration_tests/assets`?
- Could you use plugin settings to disable some systems instead of iffing in the systems?

# Todo
- Better querying
- Waiting for a condition
- Scenarios to base off of
	- Requires copying the world.
	- App is not copy or clone, maybe a world is?