# Flinging when pushing
- Landing on top of the other player has a weird pushing interaction (instantly teleports the players)
- Use hex shaped colliders
  - Rect, but push only horizontally
  - **distance to push = clamp(overlap.x+overlap.y-half_width of lower, 0, overlap.x)**
    - Half it, apply to both, this could fix different collider sizes
    - Or maybe **sum(widths) / 4** as the constant
  - Should create a tall hex shape
- Maybe if player collision used hex shaped colliders it would be easier to handle sliding off when landing on the other player
  - Player collision shouldn't affect the Y axis.
  - After player momentum is applied, the players are both moved for half of their intersection
  - If both are grounded, use collider width to adjust X
  - If one or both are airborne, use manhattan distance to calculate diamond sides and adjust X

Order of operations:
- Apply velocity
- Handle pushing
- Clamp movement to the play area
  - If a player got clamped, handle pushing somehow

- Add a component for maintaining whether or not an entity has collided with a wall
- Generalize velocity, get rid of playervelocity if possible
- FuturePosition component and a system that clamps and commits it to transform after physics pipeline

# Move velocity addition
- Back and forth dashing has mad speed for some reason
- Maybe something to do with impulse applied over time?

# QOL
- Use worldqueries like elsewhere

Maybe:
- Calculate side flips only if players don't overlap
- Apply push force only if both players are grounded.
