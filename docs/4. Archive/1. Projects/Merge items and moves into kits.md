- Merge items and moves crates into a kits crate
- Because Items need MoveIds, Moves need ItemIds, using types to link is a bit ugly
- Exports
	- [[#Kit]]
	- [[#Inventory]]
	- Move, MoveId
	- Item, ItemId (Make this into an enum as well)

# Kit
- Kit contains all the moves the character can possibly have and all the items they can roll from the shop
- Kits are immutable
- Every character has their own Kit
- Replaces old MoveBank

# Inventory
- Generic component
- Retains info on
	- what items the player has
	- how much money they have
	- what numeric buffs their items add up to
		- Like if multiple items give you shorter startup time, inventory sums them up