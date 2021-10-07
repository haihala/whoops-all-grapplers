How pressing buttons leads to a move coming out.

# Pipeline
1. main crate
	1. On startup `InputReader` components are added to players
		1. When it's added, the `InputReader` goes through a character specific function that registers some special moves to it
2. input_parsing crate
	1. Input events are registered (new controller, removed controller, input update)
	2. Controllers are added and disconnected as they ought to be
	3. Input events are parsed into `OwnedChange`s, a single change
	4. `OwnedChange`s are combined and grouped to per controller `Diff`s
	5. Diffs are applied to `InputReader`'s current pressed buttons and stick position
	6. `InputReader` looks to see if the `Diff` advances any of the registered special moves
	7. If yes, it will add an event to it's events set
	8. Events and intermittent moves are removed after a while automatically
3. main crate again
	1. `movement_executor` system reads current stick position from `InputReader` and triggers movement related functions (largely handled in physics)
	2. Character specific executors check `InputReader` events to see if any of the registered events show up

# Limitations
Currently incapable of handling:
- Formats that arent (stick position changes)(maybe button). These include:
- Charge
- Negative edge
- Delays
- Multi-button inputs

# Improvements for a later day
- [ ] Convert ownedchanges directly to diffs without merging
	- This may have a problem if the events are not in order
