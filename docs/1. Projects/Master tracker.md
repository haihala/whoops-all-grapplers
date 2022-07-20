# Backlog
## High priority
- [[Character 1]]
	- [[Animations for character 1]]

## Low priority
- [[Recovery and knockdown]]
- Constants to an env (both command line and file) parser so it doesn't change on recompile.
- [[Menus]]
- [[Dynamic move phases]]
- [[Toasts in various spots]]
- [[EX move frame perfection]]
- Character select

## Easy nibbles
- Make parrot echo inputs while recording
- Timer font is weird in full screen relative to health bars

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
	- Should you be able to cancel into specials on whiff?
		- Extra cost? Maybe that'll just get paid accidentally *all the time*
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
