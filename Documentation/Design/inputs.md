# Inputs

TODO: I don't really like Defend and Movement because they are both buttons and general concepts

| Symbol | Name           | Mapping on xbox | Mapping on PS |
| ------ | -------------- | --------------- | ------------- |
| (F)    | Fast attack    | X               | Square        |
| (V)    | Vicious attack | B               | Circle        |
| (D)    | Defend         | Y               | Triangle      |
| (M)    | Movement       | A               | X             |
| (G)    | Grab           | Right trigger   | R2            |
| (O)    | Overdrive      | Left trigger    | L2            |
| N/A    | Play           | Right bumper    | R1            |
| N/A    | Record         | Left bumper     | L1            |
| N/A    | Reset          | View button     | Share button  |

Pressing play and record resets training mode.

(Pseudo) universals using numpad notation:

- 5F is the fastest move for most characters
- 5B activates **AJB**
- **Push block**
  - 3B is downwards **push block**
  - 6B is horizontal **push block**
  - 9B is upwards **push block**
- 5M on the ground is a **super jump**
- 5M in the air will make you **hover**
- Dashes
  - 4M and 6M are **ground dashes** back and forth
  - 8 Directional **air dash**
- 5G is **grab**
- 2G, 6G, 8G, and 4G **throw** the opponent immediately in that direction
- **Overdrive**
  - If pressed in hit/block stun, **Combo breaker**
  - If pressed mid animation, **Flash cancel**
  - If none of the above, puts your character into **Overcharge**

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
  - Using a DP when a 2HP would do in SF can in tight situations be the wrong call,
    - Inputting takes time, evening out the faster startup of the DP
      - L/M/H/ DP: 3/4/5 frames vs C.HP 6
      - It takes a minimum of 3 frames to input a DP and doing it that fast is not really physically doable

So motion inputs have uses, but they also have several pain points, thus limit motion inputs per character.
Characters have a numeric score of mechanical difficulty, based on the following factors:

- Motion difficulty in a vacuum
- Move is the `best/only` solution to a `rare/common` situation

### Table of input difficulties

Every one of these ends with a button and assumes that the player is facing right, mirrored options exist,
but would needlessly flood the table so they are left out.

| Name of input                  | Numpad notation               | Difficulty in a vacuum [0-10] |
| ------------------------------ | ----------------------------- | ----------------------------- |
| Normal                         | 5                             | 0                             |
| Command normal                 | One of 4/1/2/3/6              | 1                             |
| Air normal                     | 5 (in the air)                | 1                             |
| Air command normal             | One of 4/1/2/3/6 (in the air) | 2                             |
| Charge back                    | Hold 1/4/7, 6                 | 2                             |
| Charge down                    | Hold 1/2/3, 8                 | 2                             |
| Charge up                      | Hold 7/8/9, 2                 | 3                             |
| Charge forward                 | Hold 9/6/3, 4                 | 3                             |
| Quarter circle                 | 236                           | 3                             |
| Reverse quarter circle         | 632                           | 3                             |
| Inverse quarter circle         | 896                           | 4                             |
| Inverse reverse quarter circle | 698                           | 4                             |
| Half circle forward            | 41236                         | 4                             |
| Buster forward                 | 412364                        | 6                             |
| Dragon punch forward           | 623                           | 6                             |
| Inverse dragon punch forward   | 689                           | 7                             |
| Pretzel                        | 1632143                       | 8                             |

Doubling a move adds +2, i.e. Double quarter circle = Quarter circle (3)+2 = 5

Back and forth also adds +2

Back and forth and double both add +6, so a simple Quarter circle back doubled back and forth would be 3+6=9. Order doesn't matter.

### Charge

- Holding in a cardinal direction builds charge in that direction.
  - Charge has levels 1-3
  - Just tapping in a direction gives you charge level 1
  - Holding for 0.5s gives you level 2
  - Holding for 1s gives you level 3

## Macros

Player can define their own macros. Any combination of buttons can be mapped to any special move.
These bindings are not set by default, because the game needs to be playable without them, but the game
needs to encourage binding harder things like DPs to macros.

The macro will play back the special move with frame perfect inputs, so if it includes something like an inverse quarter circle,
the move will still have a jump in it.