# Ponderings
## Combo structure
- Understandability is important
	- Learning systems over exceptions
	- -> Minimal cases lead to the smallests spreadsheets -> least exceptions
- Meaningful distinctions
	- normal/special
	- EX or no
	- Projectile/attack/throw/movement
		- Projectile/attack/throw should be on the same level
		- Dash canceling should be a thing, at least after upgrades
		- Jump canceling is a bit weird
	- Button
		- Equipment button makes it not obvious if a button is supposed to be "heavier" or "faster"
- Longest obvious cancel chain: normal->special->ex
	- Without items, jumps are normals and dashes are specials
	- After an item, forward dash gains ex canced properties (FADC)
		- Will use meter if needed

## EX moves
- Ex moves could be cancelled into anything
	- This is begging for loops
	- Considering how meter works it ought to be fine.
- Current parser system is a bit troublesome with ex
	- If buttons come in one frame apart, the game will go ahead with the non-ex version if it can
- Maybe initially you only have f/s/g and s is ex-lite
	- fs variants of special moves are items

# Objectives
- Autocorrect
	- Too early
	- Too late
- Award meter based on accuracy
	- Like in OSU
	- Perfect / Good / Nothing
- Change cancelling to the pondered system