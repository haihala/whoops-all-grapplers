Universal states with no overlap. A character is always in one of these states.

## Neutral
Just standing about

## Walking
Walking left or right

## Move
A move is in progress

## Block stun
Character has blocked and is stuck in t he blocking animation

## Hit stun
Character has been hit on the ground and is reeling. Will recover after a known amount of frames.

## Freefall
Character was launched or hit in the air. Will recover after hitting the deck.

## Grounded
Character is laying on the ground

# State transitions
All legal transitions

## Manual named transitions
- Starting an attack: neutral -> attack startup
- Blocking: neutral -> block stun
- Hit: neutral -> hit stun
- Launch: neutral -> freefall
- Land: freefall -> grounded
- Ground recovery: grounded -> neutral
- Block recovery: block stun -> neutral
- Move recovery: move -> neutral
- Hit recovery: hit stun -> neutral
