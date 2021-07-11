# Oops! All grapplers

Fighting game.

## Design

### Zen

This is mostly so I can laugh at how wrong I got them later.

1. No situation is without a tool and no tools without situations
2. Decision making over execution
3. Seeing the mechanic explains the mechanic

### Inputs

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

#### [F]

- 5F is the fastest move
- Rest are character based

#### [S]

- Extremely character based

#### [B]

- 5B activates AJB
- Push block
  - 3B is downwards push block
  - 6B is horizontal push block
  - 9B is upwards push block

#### [D]

- 5D is a super jump
- Ground dashes back and forth
- 8 Directional air dash

#### [G]

- 5G is the first step of of a two face grab
- Directional grabs throw the opponent immediately in that direction

#### [O]

- If pressed in hit/block stun, bursts
- If pressed during own animation, prc
- If opponent is in hitstun and own animation is not in progress, rrc

#### Motions

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

##### Table of input difficulties

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

### Mechanics

#### Movement
#### Attacking
#### Blocking
#### Just block
#### AJB
#### Push block
####
####
####
####