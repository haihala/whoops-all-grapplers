# Backlog
## Clearly a priority but blocked
- [[Menus]]
	- Blocked by waiting for new bevy UI (April)
- [[Gi of the old masters]]
	- Blocked by [[Simple training mode]]

## High priority
- [[Gun]]
	- Mechanism for item specific resources
	- Maybe think about item specific flags as well
	- Could the flags just be a vec of enums or something?

## Low priority
- [[Recovery and knockdown]]
- [[Lex#Heat]]

## Easy nibbles


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
- [[Testing framework v2]]
- A way to pause the game (look at inspector)
- Generic state based visibility toggler for UI components
- Think about cancellability + moves having hit
	- Currently can't cancel rest of active
- Split move list into multiple bits so you can more easily reuse phases for switches that only modify bits of a phase
- Maybe if player collision used diamond shaped colliders it would be easier to handle sliding off when landing on the other player
- Toasts
- Attributes like startup reduction and stun increase to and from inventory
- Negative edge works weird (long press will give button on both edges because head is first used to parse. Construct custom head with only the old stick.)
- Multiple projectiles from the same move don't despawn correctly on hit.
- Moves need startup to work correctly

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
