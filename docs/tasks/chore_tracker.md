# Chore tracker

For little things that don't deserve their own tickets nor are time sensitive
enough to be put on a more important list.

## Bugs

- Controllers connected after game has started show up weird in the controller claim menu
- Red paint + throws
- Stick input occasionally gets stuck
  - Replicate with 46464646... on the dpad

### Online

- Online clock is out of sync

### Watchlist

- Does input parser work after frame reset?
  - Repeat inputs from the previous round?
- Cancel windows feel really bad
- Defender can't block 2f spam and there are opener sparks (had paint)
- Players falling through the floor

## Misc improvement ideas

- Make airborne characters not turn around
  - This requires splitting facing into multiple facings
  - One for character model, one for inputs.
- Pushback should not apply to projectiles
- Newtype wrapper for frames.
  - Currently using usize, which means frames 99% of the time, but not always.
- Maybe you could make requirements dynamic as well?
