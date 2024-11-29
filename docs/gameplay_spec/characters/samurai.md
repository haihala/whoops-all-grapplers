# Samurai

## Lore

She is:

- A young but competent second generation Korean American immigrant.
- A rookie in [W.A.G.](/docs/gameplay_spec/lore/w.a.g..md)
- The point of view character for storytelling.

Her Japanese samurai gimmick is forced on her and the corporation does not see
how this may be culturally insensitive.

## Motifs

- Samurai gimmick
  - Visual indicators
    - Samurai outfit
    - Oni mask
    - [Kunai](#Kunai)s
    - Katana
  - Effectiveness through effort and practice
    - [Sharpness](#Sharpness) mechanic
- Enthusiasm
  - Dash that hops in
  - Fast forward walk

## Place in the roster

- She will be the anchor in terms of balance.
  - All other characters ought to be balanced around her
- Strengths
  - Hard reads => hit hard
  - Can snowball with [Sharpen](#Sharpen)
- Weaknesses
  - Mediocre without [Sharpness](#Sharpness) stacks
  - Limited [Kunai](#Kunai) projectiles make longer rounds hard
  - Meh pokes

## Mechanics

- Duck is abnormally low and goes under stuff

### Sharpness

- Stacking buff
- [Sharpen](#Sharpen) adds a stack
- Having stacks makes sword moves do more damage
- Resets between rounds
  - Unless you have [sword oil](/docs/gameplay_spec/items/blade_oil.md)
- Increase at round start with [smithy coupon](/docs/gameplay_spec/items/smithy_coupon.md)

### Kunai

- Limited resource used for [Kunai throw](#Kunai throw)
- Resets at the start of the round
  - By default, 2
- Upgrades
  - [Backup Kunai](/docs/gameplay_spec/items/backup_kunai.md)
  - [Kunai pouch](/docs/gameplay_spec/items/kunai_pouch.md)
  - [Kunai bandolier](/docs/gameplay_spec/items/kunai_bandolier.md)

## Moves

### Normals

#### Knee thrust

- Input: `f` while standing
- Visual: Lily st.lk, but goes a bit higher
- Function: Abare, Clash parry trigger

#### Low poke

- Input: `f` while crouching
- Visual: Urien cr.lk
- Function: Low check, combo filler

#### Falcon knee

- Input: `f` while in the air
- Visual: Nago j.k
- Function: Jump-in, pressure, air to air, combo starter
- Two hitboxes, knee and back leg
  - Knee
    - Quite active
    - Sweet spot for first active frame
- Somewhat awkward to hit up close, as there is a notable gap between the hitboxes

#### Donkey kick

- Input: `s` while standing
- Visual: Ryu donkey kick from 3s (has the step)
- Function: Long range high commitment whiff punish / Neutral skip

#### Uppercut

- Input: `s` while Crouching
- Visual: Step back leg forward into back arm uppercut
- Function: Anti-air, big combo starter
- Doesn't go that far vertically
- Negative on block

#### Foot dive

- Input: `s` while in the air
- Visual: Dr Doom foot dive
- Function: Alters jump arc, combo and pressure starter
- Can be held to alter timing or empty jump

#### Ground throw

- Input: `w` while standing
- Visual: Grab the collar, drag it down and sweep the leg
- Keeps you close for oki

#### Crouch throw

- Input: `w` while crouching
- Low hitbox
- Switches sides by swinging the opponent by their feet

#### Air throw

- Input: `w` while in the air
- Visual: Ky flippy air throw

#### Thrust

- Input: `g` while standing
- Visual: Nago f.s
- Shoulder level stab
  - Fast start up relative to range
  - Slow to recover
- Is a sword move, so it deals good damage, especially with [Sharpness](#Sharpness)

#### Sky stab

- Input: `g` while crouching
- Visual: Single diagonal upwards stab
- Premium anti-air
  - Fast start up, long range
  - Slow to recover
  - Disjointed
  - Hard to convert off of?
- Is a sword move, so it deals good damage, especially with [Sharpness](#Sharpness)

#### Air stab

- Input: `g` while airborne
- Downwards stab sort of similar to Baiken j.h, but the sword doesn't go as far down
- Easy cross-ups
- Is a sword move, so it deals good damage, especially with [Sharpness](#Sharpness)

### Specials

#### Sword stance

- Input: `214` + `f`, `s` or both
- Enters a stance where she holds the sword like Nago
- Follow ups, use negative edge to trigger, `]x[` indicates release of buttons
  - `g` - [Sharpen](#Sharpen)
  - `2]x[` - [Viper strike](#Viper strike)
  - `6]x[` - [Rising sun](#Rising sun)
  - `5]x[` - Cancel
- `f` versions are faster
- `s` versions are more damaging, but slower
- `fs` versions have invincibility (ended on cancel)
- Upgrades
  - [Smoke bomb](/docs/gameplay_spec/items/smoke_bomb.md) gives a dash
  - [Fireaxe](/docs/gameplay_spec/items/fireaxe.md) gives `6]x[` [Sword slam](#Sword slam)

#### Sharpen

- [Sword stance](#Sword stance) follow up
- Visual: Run a whetstone by the sword before putting it back
- Gain a point of [Sharpness](#Sharpness) and some [Meter](/docs/gameplay_spec/genre_mechanics/meter.md)

#### Viper strike

- [Sword stance](#Sword stance) follow up
- Visual: Lunging low sword thrust
- Hits low
- Death on block and whiff
- Knockdown on hit
  - Ground combo ender?
- Is a sword move, so it deals good damage, especially with [Sharpness](#Sharpness)

#### Rising sun

- [Sword stance](#Sword stance) follow up
- Visual: Arching sword swing
- Functions: Anti-air, air combo juggler, hard read, whiff punish
- Death on block
  - Decent range, not trivial to whiff punish
- Is a sword move, so it deals good damage, especially with [Sharpness](#Sharpness)

#### Sword slam

- [Sword stance](#Sword stance) follow up
- Hits overhead
- Slow

#### Kunai throw

- Input: `236` + `f`, `s` or `fs`
- Visual: Ibuki kunai throw in SFV
- Function: Projectile
  - Projectile with a slight arc to it
  - Consumes a [Kunai](#Kunai) on use, can't use if you have none
- Versions
  - `f` is pretty basic
  - `s` goes high up, useful for oki and space control
  - `fs` is fast and horizontal
- Upgrades
  - More [Kunai](#Kunai) per round
  - [Protractor](/docs/gameplay_spec/items/protractor.md)
  - [Mini taser](/docs/gameplay_spec/items/mini_taser.md)

## Ideas

- [Kunai](#kunai-throw)
  - Ability to pick them back up / restock
  - A second hitbox after making contact (bomb?)
  - Air version?
- [Falcon knee](#falcon-knee) back foot hitbox (needs char to not flip mid-air)
- [Sharpness](#sharpness) to binary and add more notable effects
- [Sword stance](#sword-stance)
  - Item: Hold versions that empower follow ups
    - Some ideas
      - [Rising sun](#rising-sun) - Moon launch
      - [Sword slam](#sword-slam) - Overhead turns to Unblockable
    - Maybe sharpness gives you the hold versions fast
- [Sword slam](#sword-slam)
  - Better name
