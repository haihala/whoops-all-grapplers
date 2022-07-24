#blocked by [[Move activation rework]] and [[MoveSituation extention]]

- Problem is that if the non-ex version starts, nothing is done if the second EX button comes in a frame late.
- New system (like linking and canceling) for retroactive corrections
	- Diff of phases that have been completed, correct per phase
		- Timing ougth to be trivial
		- Override animations
		- Adjust spawned boxes
		- Moves that have hit are a bit difficult
- How to tell what autocorrects into what
	- Input overlap
	- Input complexity
	- -> Add a thing into move inputs asking if one is a sub-input of the other
		- Cover cases like buster and backdash for Pot
		- Doesn't cover all, but will work for EX moves
