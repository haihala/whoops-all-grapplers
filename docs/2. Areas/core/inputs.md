# Inputs
symbol - mapping on xbox - name

- `f` - A - Fast attack
- `s` - B - Strong attack
- `g` - Y - Grab
- `e` - X - Equipment
	- [[item]]

Concerns for a later date:
- Dash macro
- EX button (f+s)
- Taunt
- Demo/training mode buttons.

If something can be done with both the stick and a button, add the ability to disable and enable both separately (strive doesn't let you remove dash from the stick even if you have the macro which causes accidental dashes)

## Motions
Motion inputs are useful for:
- Expanding move lists without adding buttons
- Balance mechanism for bigger moves
- Flow of motions can feel nice
- Interesting and genre unique mechanic for depth (Guile moving forwards loses two special moves, so is disincentivized from doing so)

Motion inputs can create frustrations due to:
- One sided feeling of their difficulty
  - You don't always notice when the opponent fucks up
- Learning curve aka "What the fuck is that" reaction to seeing a DP input.
- Not realizing some mechanical problems are decision making problems
  - Using a DP when a cr.hp would do in SF can in tight situations be the wrong call,
    - Inputting takes time, evening out the faster startup of the DP
      - L/M/H DP: 3/4/5 frames vs cr.hp 6
      - It takes a minimum of 3 frames to input a DP and doing it that fast is not really physically doable, it's rare to get it done in less than 10 frames

So motion inputs have uses, but they also have several pain points, thus limit motion inputs per character. Characters have a numeric score of mechanical difficulty, based on the following factors:

### Charge
- Holding in a cardinal direction builds charge in that direction.
- Charge is traditionally hard to get a feeling for
	- Solution 1: Levels of charge
		- Inputting the motion without holding the charge gives you something close enough
	- Solution 2: Charge bar
		- Visual indicator on your charge.
