# Backlog
## High priority
- [[GLTF helper systems]]
- [[Clash parry]]
- [[Move progress cleanup]]

## Low priority
- [[Recovery and knockdown]]
- Constants to an env (both command line and file) parser so it doesn't change on recompile.
- [[Animations]]
- [[Menus]]

## Easy nibbles
- Instead of pure phase index, use a combination of index, branch coordinates to determine where the execution of a move is at.
	- This allows to have a window to transition into another action instead of just at action boundary
	- e.g. user can tap gunshot while already shooting and the second shot comes immediately and not after a delay. Holding would still be possible.
- Player model gets clipped into the background near the corners

## Not a priority
- [[Counter hits]]
- [[Ending rounds]]
- [[Netplay]]
- [[Resources between rounds]]
- [[Rewards]]
- [[Simple training mode]]
- [[Lex]]
	- Split movelist generation function
	- Heat
- [[Testing framework v2]]
- A way to pause the game (look at inspector)
- Generic state based visibility toggler for UI components (there is a component for visibility)
- Think about cancellability + moves having hit
	- Currently can't cancel rest of active
- Maybe if player collision used diamond shaped colliders it would be easier to handle sliding off when landing on the other player
- Toasts
- Attributes like startup reduction and stun increase to and from inventory
- Negative edge works weird (long press will give button on both edges because head is first used to parse. Construct custom head with only the old stick.)
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
