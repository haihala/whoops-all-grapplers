- Maybe if player collision used diamond shaped colliders it would be easier to handle sliding off when landing on the other player
  - Player collision doesn't affect the Y axis.
  - After player momentum is applied, the players are both moved for half of their intersection
  - If both are grounded, use collider width to adjust X
  - If one or both are airborne, use manhattan distance to calculate diamond sides and adjust X
- back and forth dashing has mad speed for some reason
- Landing on top of the other player has a weird pushing interaction (instantly teleports the players)
- Use worldqueries like elsewhere

Maybe:
- Calculate side flips only if players don't overlap
- Apply push force only if both players are grounded.
- Allow intersecting, apply minecraft esque push force.

Corner cases:
- Jump over into the corner
- Jump when someone is about to land on you
