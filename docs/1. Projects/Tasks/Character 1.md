See [[Ryan]] and [[Lex]] for ideas

Animating is separated into [[Animations for character 1]], but this one does include move data.

# Normals
- 5f Slap
- 2f Low chop
- air f basic air to air
- 5s Guile burn straight
- 2s is a poor man's dp
	- launcher anti-air
- air s is a divekick
- 5g normal throw
	- Main function is to win over space
	- Can't really combo off of it, but it will yeet the opponent a good way back
- 4g backthrow
	- Switches sides
	- Reuse 5g animation
- 2g Dudley sweep
	- Easily jumpable and a bit particular with the range
	- Can combo after it if hits
- air g is an airgrab that makes you go down with the target

# Specials
- 46 f/s/g Dash punch
	- Chargeless
		- f
			- Quick
			- Comboes after all normals
		- s
			- EX cancels
			- Two hits
			- Bigger hitbox
			- Costs meter
		- g
			- Short range tackle
			- Slam the opponent down
			- On tech and hit will cross over
			- On whiff, will get punished
	- Charged
		- Requires 0.5s for full charge
			- Charge amount should be customizable in `Character`
		- Does a little hop
			- Spacing non-g versions correctly will make it hit low on most characters
		- f
			- Quick
			- Short range
		- s
			- Longer range
			- EX cancels
			- Two hits
			- Bigger hitbox
			- Costs meter
		- g
			- Tackle
			- Will throw the opponent into the air
				- Can combo after
			- On whiff, will get punished
			- On tech will cross over
- [789]6 f/s volleyball slam
	- Jump into the air and slam, mostly useful as an anti-air
	- "default" version is airborne, but when buffered you can cancel into it for a low to the ground version
	- s version
		- floats him after for more air buttons
		- Costs bar
