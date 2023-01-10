Interlocked with [[Items and the shop]]

- Layout
	- Timer top middle
	- Half the screen horizontally
	- At the top is a box that tells you about the hovered item
	- Middle is a list of available items
		- Grid where columns are categories:
		- Items are listed under in price order from the cheapest to the most expensive 
	- Player's inventory at the bottom
		- Limited in size to 4 initially
- Binds
	- Contextual button to buy or sell depending on what is hovered
	- Directions to move
	- Ready up button
	- Something to move quickly?
		- Shift like button to jump an entire segment
		- Jump doubles?
		- Option for an obscure navigation mode
			- Kinda requires [[CLI args]]

# Todo (non-obvious)
- Doen't need to be pretty
- Use first letter of an item as the icon
- Gray out items you can't afford

# Architecture brainstorming
- Centralized component to store/write/read state would be cool
	- Referencing said component may be difficult
	- Link after creation?
- How to update buttons?
- How to highlight selected option?
- Where to read input?
