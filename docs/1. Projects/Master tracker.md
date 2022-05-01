# Backlog
## Clearly a priority but blocked
- [[Menus]]
	- Waiting for new bevy UI (April)
- [[Gi of the old masters]]
	- [[Simple training mode]] would make testing a lot easier
	- [[Merge items and moves into kits]]
	- [[Non-standard input actions]]
- [[Gun]]
	- [[Resources component]]
	- [[Merge items and moves into kits]]

## High priority

## Low priority
- [[Bevy 0.7]]
- [[Recovery and knockdown]]
- Constants to an env (both command line and file) parser so it doesn't change on recompile.
- [[Stance system]]

## Easy nibbles
- Change move advancement to set values in components (like spawner and grabbable) and have other systems pick up on the values
	- This is to avoid uber long argument lists.
- Change how move starting works so that the first phase goes through the same system as the other phases
	- This would fix the bug where the first phase hitboxes etc are ignored
	- If it becomes difficult, could add a 1-frame animation as the first phase automatically.

## Not a priority
- [[3D model]]
	- [[Animations]]
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
- In move advancement, check if action changed not if index changed (this allows recursion or long recovery that can be cancelled into recursion)

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
