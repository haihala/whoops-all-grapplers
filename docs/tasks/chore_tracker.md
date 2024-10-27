# Chore tracker

For little things that don't deserve their own tickets nor are time sensitive
enough to be put on a more important list.

## Bugs

- Hit spark spawns in the wrong position for projectiles (relative to player)
- Extra kunais may not work (besides the UI)

### Watchlist

- There is an input recording strangeness, parrots don't always work, probably an ordering question.
- Throws may not work after no longer relying on animation data
- Input to cancel stance looks funky

## Misc improvement ideas

- Pushback should not apply to projectiles
- Newtype wrapper for frames.
  - Currently using usize, which means frames 99% of the time, but not always.
- Could you mirror player animation/model with shaders?
  - Boxes may be harder to draw?
- Disable parrots in prod
- Is there a more elegant way to encode concepts such as "charge move" and "meter cost"
- Maybe you could make requirements dynamic as well?
  - Encode special cancels in there
    - To make mechanics like burst and guard cancels easier
