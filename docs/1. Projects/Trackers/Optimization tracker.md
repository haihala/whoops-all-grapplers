# Input parsing
- Maybe input parsing could clone less and burrow more
- Maybe only store index in parser head

# Move data
- [x] Simple bool cancellable to phases and get cancel level from the move's cancel requirement.
- [x] System that checks when adding moves that the same key is not used many times
- Change the keys to an enum

# Testing
- Write an integration testing system
	- Specifying a test:
		- 'walk forward for a second'
		- 'attack'
		- 'assert other person has certain amount of health'
- Time crate
	- Move clock, game states and once per tick system thingy there
	- Re-export std or https://lib.rs/crates/mock_instant (for tests)
- Generic system for input parsing, reader for tests

# Workflow maybies
- Just instead of make
- Toolchain.toml

# Other
- Can you use const sin for angles in jumps yet?