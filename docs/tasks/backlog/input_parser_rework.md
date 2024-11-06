# Input parsing rework

## Remove the concept of parser heads

- Do a more traditional input buffer thing
- Inputs look back some time relative to their complexity
  - "f" will look back the bare minimum (1 frame?)
  - "236236f" will look back further

## Extend

- Re-introduce multipress inputs.
  - Handle upgrades when the buttons aren't pressed exactly on the same frame
    - Automatically kara to overlapping inputs
  - Think if Mizku sword stance should use multipress for EX version
    - It's probably cleaner to have EX be two buttons
      - Sometimes you clearly want two non-enhanced versions
      - Universal patterns good
    - The question is how the fast and strong versions should differ
      - Timings are the classic example
- Blacklist/whitelist inputs that invalidate the input
  - 236236 shouldn't trigger 66
  - Blacklist syntax uses ! prefix

### Notes from testing multipresses

- on keyboard, a 1 frame correction would probably be fine
- on a pad, some thumb positions lead to inconsistencies
- Make it a 3 frame difference window
  - if nothing happens on the first three frames than that is a-ok
