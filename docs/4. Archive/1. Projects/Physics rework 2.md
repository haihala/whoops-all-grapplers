# Flinging when pushing
- Landing on top of the other player has a weird pushing interaction
- Use hex shaped colliders
  - Rect, but push only horizontally
  - **distance to push = clamp(overlap.x+overlap.y-half_width of lower, 0, overlap.x)**
    - Half it, apply to both, this could fix different collider sizes
    - Or maybe **sum(widths) / 4** as the constant
  - Should create a tall hex shape
- Player collision shouldn't affect the Y axis.

Order of operations:
- Adjust velocity from moves
- Apply velocity
- Handle pushing, mark if pushing is happening
- Clamp y
- Clamp x
  - If pushing is happening, move both by the same amount

# Move velocity addition
- Back and forth dashing has mad speed for some reason
- Maybe something to do with impulse applied over time?

# QOL
- Use worldqueries like elsewhere
- Calculate side flips only if players are grounded

# Next up
- Visual stutter when pushing
