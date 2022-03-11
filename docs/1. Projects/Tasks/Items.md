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
- [ ] Starting items
- [ ] Roll items based on tiers
- [ ] Tier progression
	- [ ] Button to pay for a tier up
	- [ ] Cost of tiering up goes down every round and up every tier

Average greed tier progression:
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
- [x] Maintains
	- [x] Read-only catalog of [[#Item]]s
	- [x] Read-write set of ones the player has
	- [x] Player's money
- [x] Can
	- [x] Tell what items the shop can roll
	- [x] Tell the system that triggers on exiting shopping what items were recently bought so it can edit health for example

System for modifying things before round starts
- Health and meter
- Maybe other resources should be pooled together?

## Item
Knows:
- Tier
- Cost
- is_starter - can this be rolled as a starting item
- Enum that maintains state.
- Optional Phase flag, for if the item affects a move.
	- Flags for owned items should be set at activation

What items need to be able to do and proposed ways to do them:
1. Change move phases
	1. Switches for phases activate based on flags set by items
2. Grant new moves (Maybe by default you have to buy a reversal?)
	1. Have the move in move bank list an optional required item id to trigger
		1. Add to input parsing on load just like the rest of them
		2. Check if requirements are met on activator
3. Grant passive triggering abilities ([[Gi of the old masters]])
	1. Simply check on a system that the player has the item
4. Grant passive constant buffs ([[Drugs]])
	1. Change properties directly for components in a post-shopping system
5. Have internal state (Ammo for the [[Gun]])
	1. Resources required by a move are stored in a combined component
		1. Meter + bullets + other stuff
	2. Non-move stuff like the parry windows for [[Gi of the old masters]] are stored on the component itself

# Other
- [ ] Acquire currency (see [[Bonuses]])
- [x] Items crate

# Open
- How to access non-move inputs (parry shouldn't be a move)
- Should the data for stuff like parry windows be on components somehow?
	- Mapping in buying system?
- Proto buying items

# Plan
1. [x] KB button to buy a gun (copy hadouken)
2. [x] It works with the item, doesn't work without it
	1. [x] New moves
	2. [x] Move condition
3. MVP UI
4. PhaseConditions needs a more general name, it should be used for 'does the player have this item'

# Expansion for the future
- Selling items
- Item icons
- UI v2
- Move editing, resources, item components, inventory v2
- Requirements