- [x] Adding an impulse does nothing 

Goals:
- Walking is a constant speed with no acceleration
- Block knockback works for the attacker
- Air moves don't stop the character mid-air
- Walls and floors bounce a little if you impact them hard

Solution 1:
- [x] Store the previous move id
- [x] If walking, set speed
- [x] If no move id set or it matches, set speed
- [x] If move id doesn't match, add to speed.

Also:
- [x] Can't go through the other player
- [x] Can push the other player
- [x] Jumps as moves
	- [x] Neutral
	- [x] Diagonals
	- [x] Update mobility
	- [x] Remove event
	- [x] Remove prejump state
	- [x] Instead of ambiguous holding mechanic, do a basic ass superjump
- [x] Gravity

Bugs:
- [x] Jump doesn't come out
- [x] Changing walking direction does nothing