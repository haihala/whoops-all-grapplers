# Chore tracker

For little things that don't deserve their own tickets nor are time sensitive
enough to be put on a more important list.

## Bugs

- Walking animation gets stuck sometimes
  - When walking against the enemy towards the corner
  - Sometimes when button is held for a while, walking state gets stuck
  - May be a keyboard thing

### Watchlist

- Extra kunais may not work (besides the UI)
- There is an input recording strangeness, parrots don't always work, probably an ordering question.
- Throws may not work after no longer relying on animation data

## Misc improvement ideas

- Pushback should not apply to projectiles
- Newtype wrapper for frames.
  - Currently using usize, which means frames 99% of the time, but not always.
- Disable parrots in prod
- Maybe you could make requirements dynamic as well?
