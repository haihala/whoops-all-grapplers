Universal states with no overlap. A character is always in one of these states.

# Neutral
# Attack startup
# Attack active
# Attack recovery
# Block stun
# Hit stun
# Freefall
# Grounded
# State transitions
All legal transitions

## Manual named transitions
- Starting an attack: neutral -> attack startup
- Blocking: neutral -> block stun
- Hit: neutral -> hit stun
- Launch: neutral -> freefall
- Land: freefall -> grounded

## Automatic transitions:
- startup -> active
- active -> recovery

### Return to neutral
- recovery
- block
- hit stun
- grounded
