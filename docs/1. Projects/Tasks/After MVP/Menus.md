For stuff like pause and main menus

- Represent with a nested enum
- Add ability to recognize directional taps in input parser
- Load clear moves in parser
- UI (using bevy_egui)
	- Usable through the parser
- A way to ignore menus and jump straight into a testing scenario

Rough outline:
- Main
	- Play
		- Local
		- Online
		- Back
	- Learn 
		- Lab -> Character select
		- Tutorial
		- Glossary
	- Options
		- (extend as relevant)
		- back
	- Quit
		- Yes -> Quits the game
		- No -> Back to main