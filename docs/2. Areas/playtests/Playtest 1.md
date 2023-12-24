[[Feedback round 1]]


Notes:

- Crashes
	- Three spots, marked in the code with TODO comments
- Thumbtacks affect chip
- Characters occasionally face the wrong way in the preround
- Throw is jank
- Selling immediately after buying should be free (undo button)
- Jitter, potentially related to the paint cans
- Blocking
	- While in the air
	- While getting hit
- Pilebunker reads as a grab
- Pilebunker doesn't mirror it's velocity correctly (always launches left)
- Meter self-fills in the late game
- Many things don't hit crouchers
	- This may not be a bug but a feature
	- Including throws
- Accidental sharpenings
	- g ought to be a more basic action and sharpen ought to be a move
- Inventory doesn't refresh what a player can buy when re-entering shop


Ideas for non-obvious fixes:

- Ground throw (crouch throw)
- Sharpen to something like a half circle input
    - Get rid of the gimmick button
        - Replace with dash?
        - Another attack button?
    - New gimmick (sword for mizku)
        - Move sharpen to like a quarter circle back special
        - Add a standing slash and a crouching slash
        - All slashes scale in damage from sharpen stacks
- Rotator system
    - Make it run in pre-round
    - Maybe making it less precise will fix some jank elsewhere
- Items 
    - On average should be way more expensive
    - Store previous inventory state and add a bind to revert to it (undo)
    - There ought to be more upgrades of upgrades
- Reset jump back to default (current superjump), add a short hop with input `[123456][789][123]`
    - Core part of defense is jumping lows and ducking highs


