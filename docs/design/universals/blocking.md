# Blocking

Hold back to block. Every move does lethal chip damage. Blocking is hitbox based. A blocking character has a blockbox similarily to a hitbox and if a move makes contact with the blockbox before touching a hurtbox, it is blocked. The blockbox is initially created when an attack would hit but the player is holding back. Holding down back places the blockbox differently to holding just back.

## Parry

Quickly tapping forwards before blocking will result in a parry. For most characters, parrying will negate chip and increase pushback.

You can't parry mid blockstring. The window only starts if you aren't blocking while doing the inputs.

TODO: Window size
Implementation: Tapping forward starts a window, if during this window an attack is blocked, it is parried instead.

You can parry a parry.