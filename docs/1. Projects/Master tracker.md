# Backlog
## Blocked
- [[GLTF helper systems]] - Needs [[Animations]]
- [[Animations]] - Needs [[Character 1]]

## High priority
- [[Character 1]]

## Low priority
- [[Recovery and knockdown]]
- Constants to an env (both command line and file) parser so it doesn't change on recompile.
- [[Menus]]
- Could there be a property generic that evaluates into a value based on a FnOnce that takes an inventory
	- Maybe you could specify a base phase and a function that transforms that based on the situation
	- This could make it so you don't need a global store for all stats and representing phases that get influence by multiple items easier.
	- Move away from switches for nearly identical phases
- Add toasts explaining why you got hit
	- Recovering
	- Low
	- Overhead
	- Simply not blocking
- [[EX move frame perfection]]

## Easy nibbles
- Make parrot echo inputs while recording
- Instead of pure phase index, use a combination of index, branch coordinates to determine where the execution of a move is at.
	- This allows to have a window to transition into another action instead of just at action boundary
	- e.g. user can tap gunshot while already shooting and the second shot comes immediately and not after a delay. Holding would still be possible.
- Timer font is weird in full screen relative to health bars
- Moves may not cancel into themselves.

## Not a priority
- [[Counter hits]]
- [[Ending rounds]]
- [[Netplay]]
- [[Resources between rounds]]
- [[Rewards]]
- [[Lex]]
	- Split movelist generation function
	- Heat
- [[Testing framework v2]]
- A way to pause the game (look at inspector)
- Generic state based visibility toggler for UI components (there is a component for visibility)
- Think about cancellability + moves having hit
	- Currently can't cancel rest of active
- Attributes like startup reduction and stun increase to and from inventory
- [[Stance system]]
- [[Extend moves]]
- Figure out how to get rid of panics while despawning hitboxes

# Other trackers
![[Bug tracker]]

![[Optimization tracker]]
![[MVP]]

# Overarching plan
- [ ] [[MVP]] (Not a rushed one, but one that is maintainable)
- [ ] Playtest
- [ ] Audiovisual upgrade
- [ ] More playtest
- [ ] [[Release]]
- [ ] Adjust based on stranger feedback
