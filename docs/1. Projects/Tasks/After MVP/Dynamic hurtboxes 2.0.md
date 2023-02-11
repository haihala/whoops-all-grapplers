See [[Dynamic hitboxes and hurtboxes]]
- Padding / min size
	- Atm sometimes when joints align, some boxes get quite thin
	- Head isn't great
	- Back is basically nonexistent
- Instead of an axis aligned box, have a diagonal rectangle from one focus to the other with the given thickness.
- Make sense of how offsets work when you have a joint as a hitbox root
	- May just be that the sprites are weird