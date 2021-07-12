# Oops! All grapplers

Fighting game. Shortened to oag, pronounced 'waag'.

## Zen

This is mostly so I can laugh at how wrong I got them later.

Major:

1. No problem is without a tool and no tools without a problem
2. Decision making over execution
3. Seeing the mechanic explains the mechanic

Minor:

1. If a button does something sometime, it should do something in neutral.
2. If you can't learn a mechanic outside of a match, it shouldn't exist
3. If a cool underused mechanic can get highlighted easily
   1. For example, space usage, usually fg stages are very plain.
      1. Corners suck to be in and the midscreen is where neutral happens is usually the extent of space usage.
      2. Maybe including minor stage elements would make for a more dynamic game

## Inputs

| Symbol 	| Name         	| Mapping on xbox 	| Mapping on PS 	|
|--------	|--------------	|-----------------	|---------------	|
| [F]    	| Fast attack 	| X               	| Square        	|
| [S]    	| Special      	| B               	| Circle        	|
| [B]    	| Block        	| Y               	| Triangle      	|
| [D]    	| Dash         	| A               	| X             	|
| [G]    	| Grab         	| Right trigger   	| R2            	|
| [O]    	| Overdrive    	| Left trigger    	| L2            	|
| N/A    	| Play         	| Right bumper    	| R1            	|
| N/A    	| Record       	| Left bumper     	| L1            	|

Pressing play and record resets training mode.

(Pseudo) universals using numpad notation:

### [F]

- 5F is the fastest move
- Rest are character based

### [S]

- S for special is a bad name, due to the concept of special moves meaning motion inputs
- Extremely character based

### [B]

- 5B activates AJB
- Push block
  - 3B is downwards push block
  - 6B is horizontal push block
  - 9B is upwards push block

### [D]

- 5D is a super jump
- Ground dashes back and forth
- 8 Directional air dash

### [G]

- 5G is the first step of of a two face grab
- Directional grabs throw the opponent immediately in that direction

### [O]

- Always uses stamina (name and symbol depend on what stamina will be called)
- If pressed in hit/block stun, bursts
- If pressed during own animation, prc
- If neither of the previous conditions applies, makes your character have the overdrive state
  - Empowers attacks with more damage in general
  - Some moves gain extra frame advantage, some became launchers, some get new properties entirely

### Motions

Motion inputs are useful for:
- Expanding move lists without adding buttons
- Balance mechanism for bigger moves
- Flow of motions can feel nice

Motion inputs can create frustrations due to:
- One sided feeling of their difficulty
  - You don't always notice when the opponent fucks up
  - Fucking up yourself will usually scrap existing plans
- Learning curve aka "What the fuck is that" reaction to seeing a DP input.
  - Only really relevant for very new players
- Not realizing some mechanical problems are decision making problems
  - Using a DP when a 2HP would do in SF can in tight situations be the wrong call,
    - Inputting takes time, evening out the faster startup of the DP
      - L/M/H/ DP: 3/4/5 frames vs C.HP 6
      - It takes a minimum of 3 frames to input a DP and doing it that fast is not really physically doable

So motion inputs have uses, but they also have several pain points, thus limit motion inputs per character.
Characters have a numeric score of mechanical difficulty, based on the following factors:

- Motion difficulty in a vacuum
- Move is the `best/only` solution to a `rare/common` situation

#### Table of input difficulties

Every one of these ends with a button

| Name of input                                  | Numpad notation               | Difficulty in a vacuum [0-10] |
| ---------------------------------------------- | ----------------------------- | ----------------------------- |
| Normal                                         | 5                             | 0                             |
| Command normal                                 | One of 4/1/2/3/6              | 1                             |
| Air normal                                     | 5 (in the air)                | 1                             |
| Air command normal                             | One of 4/1/2/3/6 (in the air) | 2                             |
| Quarter circle forwards                        | 236                           | 3                             |
| Quarter circle back                            | 214                           | 3                             |
| Inverse quarter circle forwards                | 896                           | 4                             |
| Inverse quarter circle back                    | 874                           | 4                             |
| Reverse quarter circle forwards                | 632                           | 3                             |
| Reverse quarter circle back                    | 412                           | 3                             |
| Inverse reverse quarter circle forwards        | 698                           | 4                             |
| Inverse reverse quarter circle back            | 478                           | 4                             |
| Double quarter circle forwards                 | 236236                        | 4                             |
| Double quarter circle back                     | 214214                        | 4                             |
| Double inverse quarter circle forwards         | 896896                        | 5                             |
| Double inverse quarter circle back             | 874874                        | 5                             |
| Double reverse quarter circle forwards         | 632632                        | 4                             |
| Double reverse quarter circle back             | 412412                        | 4                             |
| Double inverse reverse quarter circle forwards | 698698                        | 5                             |
| Double inverse reverse quarter circle back     | 478478                        | 5                             |
| Zigzag quarter circle forwards                 | 23632                         | 4                             |
| Zigzag quarter circle back                     | 21412                         | 4                             |
| Zigzag inverse quarter circle forwards         | 89698                         | 5                             |
| Zigzag inverse quarter circle back             | 87478                         | 5                             |
| Reverse zigzag quarter circle forwards         | 63236                         | 4                             |
| Reverse zigzag quarter circle back             | 41214                         | 4                             |
| Reverse zigzag inverse quarter circle forwards | 69896                         | 5                             |
| Reverse zigzag inverse quarter circle back     | 47874                         | 5                             |
| Charge back                                    | Hold 1/4/7, 6                 | 2                             |
| Charge down                                    | Hold 1/2/3, 8                 | 2                             |
| Charge up                                      | Hold 7/8/9, 2                 | 3                             |
| Charge forward                                 | Hold 9/6/3, 4                 | 3                             |
| Half circle forward                            | 41236                         | 4                             |
| Half circle back                               | 63214                         | 4                             |
| Buster forward                                 | 412364                        | 6                             |
| Buster back                                    | 632146                        | 6                             |
| Dragon punch forward                           | 623                           | 6                             |
| Dragon punch back                              | 421                           | 6                             |
| Inverse dragon punch forward                   | 689                           | 7                             |
| Inverse dragon punch back                      | 487                           | 7                             |
| Pretzel forwards                               | 1632143                       | 10                            |
| Pretzel back                                   | 3412361                       | 10                            |

#### Charge

- Diagonal charges?
- Multi level charges (this is the solution to accidental 6h when you wanted hammer fall)
  - Just tapping back and forth releases a level 0 move, which is intentionally lacking
  - Charge level goes up every 0.5s (each direction individually)

## Mechanics
### Movement
### Attacking
### Blocking
### Just block
### AJB
### Push block
### Momentum/Velocity/Drive/Gumption
Used to be stamina, it's what the meter is called and it's used for a variety of things.

Name should explain why and how you occasionally need to recharge it.

- Integer, max 3-5 depending on character
- Rarely more than 3 used at once
- Used by overdrives and special moves

### Combo scaling

Initially no, but it this becomes a problem add it in. Without supers it could be fine as is.

## Aesthetics

- low poly
- stylized
- somewhat pastel, punch planet-y palette
- Thinking neo tokyo vibes, maybe position in the early 80s so you can have nam vets and shit

# Glossary

# Characters
## Working on it

- Grappler with a grenade salvo on their back
  - great at melee, better at far off range.

## Ideas for later
A move that lets you cancel a backdash to a double forward dash.
- Goatfatal christmas as opposed to ramlethal valentine
- Arrogant swordsmith
  - Many different swords. Leaves a mark on you with one where they bet some life they can get a hit in fast.
- Footgun - a character with many footguns in the kit and literal guns for feet.
  - Joke character with ridiculously needlessly complex inputs
    - Thinking like 5xQCF or pentagram(927319) tier shit
- The mechanical ceiling
  - Many moves with frame perfect timings
  - Theme around music I think, musician can quip about getting good
- Axl but if opponent is too close you leash to them and maybe lose access to some attacks
- Ultimate shield
  - A designated turtle with several counters and get off me moves.
  - Very important, no tools beyond mid range to retaliate
  - Health regen?
  - How do they win?
- Air specialist
  - Up charges
- A character with actually decent install
  - Thinking barbarian rage that makes it their turn for like a third of the round straight, but comes with a heavy drawback later