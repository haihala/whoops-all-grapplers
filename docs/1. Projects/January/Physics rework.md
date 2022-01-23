- [ ] Adding an impulse does nothing 
- [ ] Move mobility should add to velocity, not set velocity (this is why knockback doesn't work, recovery frames set movement to zero)
- [ ] Think about what overrides what
	- [ ] Jumping with diagonals is a bit weird atm, it should override
	- [ ] Walking should override
	- [ ] Everything else should add
	- [ ] Make sure walking doesn't override knockback

Goals:
- Walking is a constant speed with no acceleration
- Block knockback works for the attacker
- Air moves don't stop the character mid-air
- Walls and floors bounce a little if you impact them hard