# Meta
- Items should be
	- Interesting
	- Non-mandatory seeming
	- Complexity increasing
- Do this with lateral items.
- List possible problems to have that could be solved with items.
	- There should be an item that kinda solves that problem for every problem
	- Several, likely

List of problems:
- Can't block the mixup
	- strike-throw
	- high-low
	- left-right
- Zoning
	- Can't get past
	- Can't keep at bay
- Pressure
	- Can't escape
	- Can't find an opening
- Neutral
	- Can't reach
	- Can't force whiffs
- Can't gain resources
	- Money
	- Meter
- Health
	- Can't kill fast enough
	- Can't survive their combos

How do you force a mistake
- Fundimentally, bait and read
	- Bait them by fainting weakness
	- Read when they are going to bite
- Canceling an unsafe move into a more unsafe move
	- Bait by not doing the cancel
	- Read when they are going to bite, do the cancel
	- Cash out
- Open
	- Why would someone bite?
		- Frustration
		- Desperation
		- Naivety
	- Trying to force a mistake can in an of itself be a mistake if the receiving player has an out the attacker isn't aware of.
		- Items serve two purposes, give outs and create uncertain situations

# Buy menu
- [ ] Open menu between rounds
- [ ] Sketch out a UI
	- [ ] Must be able to show two adjacent
	- [ ] Must show purchasable items
	- [ ] Must show owned items
	- [ ] Must have a box for info about hovered item
- [ ] Move cursor between menu items
- [ ] Buy and sell items
- [ ] Hover items to show what they do
- [ ] Button to say you are done, start next round when both players are done
	- [ ] Online will have a timer

# Item roll tables
- [ ] Items are sorted onto tiers
- [ ] Starting items
- [ ] Roll items based on tiers
- [ ] Tier progression

Tiers and rounds:
| Round | Min tier | Max tier |
| ----- | -------- | -------- |
| 1     | special  | special  |
| 2     | 1        | 1        |
| 3     | 1        | 2        |
| 4     | 1        | 2        |
| 5     | 1        | 3        |
| 6     | 1        | 3        |
| 7     | 2        | 4        |
| 8     | 2        | 4        |
| 9     | 3        | 5        | 

Max tier = round / 2, rounded up
Min tier = max tier - 2, minimum of 1

# Inventory component
- [ ] Add items to a list on buy
- [ ] Maintains
	- [ ] Read-only catalog of [[#Item]]s
	- [ ] Read-write set of ones the player has
	- [ ] Player's money
- [ ] Can
	- [ ] Tell what items the shop can roll
	- [ ] Warn of circular dependencies

## Item
Knows:
- Tier
- Cost
- is_starter - can this be rolled as a starting item
- can_sell - can the item be sold back
- Component to put on the player
	- For keeping track of stuff
	- An empty struct for most items
	- Implements a function that is ran when bought. Takes in a mutable:
		- Function handle to add a move
		- Health
		- Meter
	- Another function for when it is sold that undoes the first.
	- Functions don't touch:
		- Money
		- Inventory
- ID (Typeid of the component)

What items need to be able to do and proposed ways to do them:
1. Change move phases
	1. [[Metaphases to moves]] switch that takes in phases and triggers
		1. enum: item(id), on-hit, on-block, on-whiff, when low on health...
		2. Potentially problematic when there are a lot of triggers
	2. Change the move in the bank.
		1. What if multiple items want to change the same phase.
2. Grant new moves (Maybe by default you have to buy a reversal?)
	1. Have the move in move bank list an optional required item id to trigger
		1. Add to input parsing on load just like the rest of them, just never trigger it.
	2. Add the move to input parsing and the bank when the item is bought
		1. What if you forget a place to update
3. Grant passive triggering abilities ([[Gi of the old masters]])
	1. Simply check on a system that the player has the item
4. Grant passive constant buffs ([[Drugs]])
	1. Buff collector that effects are filtered through
	2. Change properties directly
		1. Health, Sweeping changes to all moves
5. Have internal state (Ammo for the [[Gun]])
	1. Generic resources component that has a tracker for all of the possible resources a character can have
	2. Have the item be a struct with fields, "Item bank" holds the metadata about buying and showing up in the shop


# Other
- [ ] Acquire currency (see [[Bonuses]])
- [ ] Items crate
- [ ] Tests that buying and selling an item is idempotent on whatever the buy and sell functions take in.