# Item design

This document gives guidelines for item desing. It is a way to think.
See [Tool design](/docs/gameplay_spec/guides/tool_design.md)

## What an item can do

- In general, one of or both of:
  - Add a new way to tackle a problem
  - Add greater reward for a situation
- Specifically
  - Add new moves
  - Increase basic stats, like
    - Health
    - Walk speed
  - Add new properties to moves, like
    - Invincibility on back dash startup
    - Armor on a reversal

## What problems are items for

- Can't avoid the mixup
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

## Designing good items

- Fun to use
  - Useful
  - Changes what the character can do in a fun way
- Fun to have used on you
  - Doesn't lead into check-mate situations
  - Works because they forced you to make a mistake
- Simple to understand
  - Explainable with one or two lines of text
  - As clear as possible when you see it in action
- An interesting decision to buy
  - A specific item is rarely mandatory
  - Problems have multiple solutions
  - Rarely does an item solve just one problem
    - Which solution has the most beneficial side effects is an interesting choice
  - Opportunity cost

## Pricing

Assertions:

- Save the whole game -> you should be able to afford the most expensive basic item
  - Limit to basic, since you shouldn't be able to afford the final
    [Thumbtack](/docs/gameplay_spec/items/thumbtacks.md) level by just losing
- You should be able to buy at least two things on the first round
- On a round 5 (4th shop), you should be able to afford any mid level item
  without saving
- Consumables should be on the cheaper side

Current economical situation:

- Shop 1 (1-0)
  - Winner: 700
  - Loser: 500
- Shop 2 (1-1 or 2-0)
  - Winner: 1000
  - Loser: 800
  - Accumulated:
    - 1-1: Both have 1500
    - 2-0: winner has 1700, loser has 1300
- Shop 3 (2-1)
  - Winner: 1300
  - Loser: 1100
  - Accumulated:
    - 2-1: winner has 2800, loser has 2600
- Shop 4 (2-2)
  - Winner: 1600
  - Loser: 1400
  - Accumulated:
    - 2-2: Both have 4200

Pricing guidelines:

- Small consumable: 100
- Small persistent: 250
- Medium consumable: 400 (not sure we have these)
- Medium persistent: 600
- Big persistent: 1000
