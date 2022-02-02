- [ ] Acquire currency (see [[Bonuses]])

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

# Wallet component
- [ ] Add item components to the player on buy
- [ ] Remove item components on selling
