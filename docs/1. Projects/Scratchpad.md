- Add a button where the character says "Nice"
	- Could have variations for when you are blocking or getting hit
- Taunting is how you cash out combo damage to money
- Polish: make parser heads peek one forward and maybe remember that and/or skip requirements
- Just put all moves in a massive enum
	- They already basically are, except in a ghetto way.
- Turning system should only work when the characters are on the ground
- A method to 'upgrade' moves with complete inputs
	- quarter+f+s should override quarter+f if it comes in early enough
- Cumulative hitstun?
	- Paritally cumulative hitstun?
	- Attach to an [[Items]]?
- A proper system for setting animations rolling
	- Checking for overlap should be done at source, not in the helper
	- Looping, speed, start frame and the animation itself all in one struct

# [[Mizku]] special and gimmick clarification
## Sway
- Uses
	- Mixups like Fujin
	- Evasion / Counter hit bait like Dandy step
	- Stance cancels like Mist finer
- Flowcharts
	- [x] Starter:
		- Sway back, Dandy ✅
		- Normal - 214f
		- Enhanced - 214s uses one bar for
			- Some i-frames
			- More distance
			- Better follow ups
		- If no further input is given, returns to neutral after a while
	- Mid point
		- [x] Sway Forward - Press the button that wasn't the starter
			- If starter was enhanced, this will go further
		- [x] Cancel - G
			- Can do it at any point before an ender
			- If starter was enhanced, this is instant
			- Mist finer ✅
	- Ender (After forward movement)
		- [x] Mixups (fujin ✅)
		- [x] Low - 2w
			- Scum dipper slide
		- [x] Overhead - 6w
			- SF3 Ryu 6mp (Same as in other games)
		- [x] Mid - W
			- Pilebunker
- Upgrade ideas
	- Jump cancellability
	- Re-dash
		- Lets you cancel either version before an ender to enhanced back sway
	- Some components of enhanced version
	- Just frame timings like SFV Karen Tenko
 - Open
	- Should there be a safe ender? - No, you can G out and go for a 5f
	- Commitment
		- Going for an ender and 

Actions:
- Rename upwards slash to rising sun
- Sharpen
	- Give meter on completion
	- Sharpness is a damage boost to rising sun



















