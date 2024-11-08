# Input parsing rework

## Extend

- Handle upgrades when the buttons aren't pressed exactly on the same frame
  - Automatically kara to overlapping inputs
- Blacklist/whitelist inputs that invalidate the input
  - 236236 shouldn't trigger 66
  - Blacklist syntax uses ! prefix?

### Notes from testing multipresses

- on keyboard, a 1 frame correction would probably be fine
- on a pad, some thumb positions lead to inconsistencies
- Make it a 3 frame difference window
  - if nothing happens on the first three frames than that is a-ok
