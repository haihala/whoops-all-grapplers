- Timer font is weird in full screen relative to health bars
- There is a timing bug, presumably when you do a charge move (and consume charge) on the same frame that charge would be lost naturally
- Weird position offset after reset could be root mover moving an intermediate object
- In a mirror, opponents joints are used as spawn targets? Only happened once.
- Hitboxes flash in strange places
	- Initial offset wrong, update system works?
	- System ordering thing
- Box visualizations occasionally jitter (specifically when walking, probably a system ordering thing)

# Before playtest

# Possibly incidentally fixed?
- Figure out how to get rid of panics while despawning hitboxes
- Jumping makes you unable to do any moves
- There is an input recording strangeness, parrots don't always work, probably an ordering question.

# Investigable discoveries
- Are mutators used?