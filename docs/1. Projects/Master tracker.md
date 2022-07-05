# Backlog
## High priority
- [[Clash parry]]
- [[GLTF helper systems]]

## Low priority
- [[Recovery and knockdown]]
- Constants to an env (both command line and file) parser so it doesn't change on recompile.
- [[Animations]]
- [[Menus]]

## Easy nibbles
- Instead of pure phase index, use a combination of index, branch coordinates to determine where the execution of a move is at.
	- This allows to have a window to transition into another action instead of just at action boundary
	- e.g. user can tap gunshot while already shooting and the second shot comes immediately and not after a delay. Holding would still be possible.
- Timer font is weird in full screen relative to health bars
- Moves may not cancel into themselves.
- Pushing causes mild visual stutter

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
- Toasts
- Attributes like startup reduction and stun increase to and from inventory
- [[Stance system]]
- [[Extend moves]]

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
