# Input parsing
- Maybe input parsing could clone less and burrow more
- Maybe only store index in parser head

# Move data
- Simple bool cancellable to phases and get cancel level from the move's cancel requirement.
- System that checks when adding moves that the same key is not used many times
	- Maybe change the keys to an enum while at it

# Testing
- Write tests to the point where a list of inputs goes in and then you can make assertions on the world. 
- https://lib.rs/crates/mock_instant instead of sleep
- Generic system for input parsing, reader for tests

# Other
- Type alias for shorter state types
- Can you use const sin for angles in jumps yet?