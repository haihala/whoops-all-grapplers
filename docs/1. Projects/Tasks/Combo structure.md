# Ponderings
## Combo structure
- Understandability is important
	- Learning systems over exceptions
	- -> Minimal cases lead to the smallests spreadsheets -> least exceptions
- Meaningful distinctions
	- normal/special
	- Amount of bar used
	- Projectile/attack/throw/movement
		- Projectile/attack/throw should be on the same level
		- Dash canceling should be a thing, at least after upgrades
		- Jump canceling is a bit weird
	- Button
		- Equipment button makes it not obvious if a button is supposed to be "heavier" or "faster"

Boils down to two rules
1. Normals cancel into specials
2. Moves cancel into ones with a higher bar consumption

Those two rules will mean that (eventually), normal, special, ex, super is completely sensible and logical.

Notes:
- Without items, jumps are normals and dashes are specials
- After an item, forward dash can use a bar (FADC)
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
- [ ] Autocorrect
	- [ ] Too early
	- [ ] Too late
		- [ ] EX
- [ ] Award meter based on accuracy
	- [ ] Like in OSU
	- [ ] Perfect / Good / Nothing
- [x] Change cancelling to the pondered system