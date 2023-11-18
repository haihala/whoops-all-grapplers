- Timer font is weird in full screen relative to health bars
- There is a timing bug, presumably when you do a charge move (and consume charge) on the same frame that charge would be lost naturally
- Weird position offset after reset could be root mover moving an intermediate object
- New box visualization occasionally panics
	- Tends to happen when attacking
	- It also somewhat flickers constantly
- In a mirror, opponents joints are used as spawn targets? Only happened once.

# Before playtest
- Kunai projectile is invisible (or just really small?)


# Possibly incidentally fixed?
- Figure out how to get rid of panics while despawning hitboxes
- Jumping makes you unable to do any moves

# Investigable discoveries
- Are mutators used?