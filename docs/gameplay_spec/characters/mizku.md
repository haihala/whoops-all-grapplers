**Mizuki**. In Japanese the word _mizu_ means “felicitous omen, auspicious” and the word ki means “hope,” forming this very cool and beautiful girls’ name.

Nicknamed Mizku because it sounds like Masku which is funny Tekken meme.
# Lore
She is a young but competent second generation Korean American immigrant. She's a rookie in [W.A.G.](/docs/gameplay_spec/lore/w.a.g..md) and thus often agrees to do stuff that she really shouldn't including her Japanese samurai gimmick. 

# Motifs
- Samurai gimmick
	- Visual indicators
		- Samurai outfit
		- Oni mask
		- [#Kunai](/#Kunai)s
		- Katana
	- Effectiveness through effort and practice
		- [#Sharpness](/#Sharpness) mechanic
		- Kurosawa-esque staredowns
- Born and raised in the states
	- Laid back, take cues from Baiken in animations.
	- Occasionally brash, can bite her in the ass
		- Mechanics encourage you to be a bit cheeky
- Her samurai gimmick comes in part from [W.A.G.](/docs/gameplay_spec/lore/w.a.g..md), she is clearly rebelling against it

# Place in the roster
- She will be the anchor in terms of balance.
	- All other characters ought to be balanced around her 
- Strengths
	- Hard reads => hit hard
	- Snowballs with [#Sharpen](/#Sharpen)
- Weaknesses
	- Effectiveness depends on [#Sharpness](/#Sharpness) stacks
	- Limited [#Kunai](/#Kunai) projectiles make longer rounds hard
	- Meh pokes

# Mechanics
- Duck is abnormally low and goes under stuff

## Sharpness
- Stacking buff
- [#Sharpen](/#Sharpen) adds a stack
- Having stacks makes sword moves do more damage
- Resets between rounds

## Kunai
- Limited resource used for [#Kunai throw](/#Kunai throw)
- Resets at the start of the round
	- By default, 1
- Upgrades
	- [Backup Kunai](/docs/gameplay_spec/items/backup_kunai.md)
	- [Kunai pouch](/docs/gameplay_spec/items/kunai_pouch.md)
	- [Kunai bandolier](/docs/gameplay_spec/items/kunai_bandolier.md)

# Moves (built-in)
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
	- Back leg
		- Can hit cross up similar to Kum j.d
- Somewhat awkward to hit up close, as there is a notable gap between the hitboxes

#### Donkey kick
- Input: `s` while standing
- Visual: Ryu donkey kick from 3s (has the step)
- Function: Long range high commitment whiff punish / Neutral skip

#### Uppercut
- Input: `s` while Crouching
- Visual: Step back leg forward into back arm uppercut
- Function: Anti-air, combo starter
- Doesn't go that far vertically
- - on block

#### Foot dive
- Input: `s` while in the air
- Visual: Dr Doom foot dive
- Function: Alters jump arc, combo and pressure starter
- Can be held to alter timing or empty jump
- Upgrades
	- [Space suit boots](/docs/gameplay_spec/items/space_suit_boots.md)

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
- Is a sword move, so it deals good damage, especially with [#Sharpness](/#Sharpness)

#### Sky stab
- Input: `g` while crouching
- Visual: Single diagonal upwards slash with a bit of a swing
- Premium anti-air
	- Fast start up, long range
	- Slow to recover
	- Disjointed
	- Hard to convert off of?
- Is a sword move, so it deals good damage, especially with [#Sharpness](/#Sharpness)

#### Air stab
- Input: `g` while airborne
- Downwards stab sort of similar to Baiken j.h, but the sword doesn't go as far down
- Easy cross-ups
- Most damaging jump-in before [Space suit boots](/docs/gameplay_spec/items/space_suit_boots.md)
- Is a sword move, so it deals good damage, especially with [#Sharpness](/#Sharpness)


### Specials
#### Sword stance
- Input: `214` + `f` (or `s`)
- Enters a stance similar to [Azami from Xrd Baiken](https://www.dustloop.com/w/GGXRD-R2/Baiken#Azami)
- Follow ups
	- `g` - [#Sharpen](/#Sharpen)
	- `f` - [#Viper strike](/#Viper strike)
	- `s` - [#Rising sun](/#Rising sun)
- If no follow up is input relatively quickly, it recovers naturally.
	- This makes some moves like [#Sky stab](/#Sky stab) safer
- Upgrades
	- Can back dash out of the stance for an evasive recovery
	- `s` version invincibility
		- Costs bar
		- Has invincibility from frame 1
		- More invincibility with further upgrades
	- `s` version follow-up amplification
		- Costs bar
		- Improves all follow-ups
			- [#Sharpen](/#Sharpen) gives two stacks
			- [#Viper strike](/#Viper strike) and [#Rising sun](/#Rising sun)
				- Deal more damage
				- Have better combo properties
				- Push back on block to make them safer
	- Air version?
	
#### Sharpen
- Input: `g` while in [#Sword stance](/#Sword stance)
- Visual: Run a whetstone by the sword before putting it back
- Gain a point of [#Sharpness](/#Sharpness) and some [Meter](/docs/gameplay_spec/genre_mechanics/meter.md)

#### Viper strike
- Input: `f` while in [#Sword stance](/#Sword stance)
- Visual: Lunging low sword thrust
- Hits low
- Death on block and whiff
- Knockdown on hit
	- Ground combo ender
- Is a sword move, so it deals good damage, especially with [#Sharpness](/#Sharpness)

#### Rising sun
- Input: `s` while in [#Sword stance](/#Sword stance)
- Visual: Arching sword swing
- Functions: Anti-air, air combo ender, hard read, whiff punish
- Death on block and whiff
	- Decent range, not trivial to whiff punish
- Is a sword move, so it deals good damage, especially with [#Sharpness](/#Sharpness)

## Kunai throw
- Input: `236` + `f` or `s`
- Visual: Ibuki kunai throw in SFV
- Function: Projectile
	- Relatively fast projectile with a slight arc to it
	- Consumes a [#Kunai](/#Kunai) on use, can't use if you have none
- `s` version throws two in a fan
- Upgrades
	- More [#Kunai](/#Kunai) per round
	- Control (joystick position based, maybe several levels)
		- Ability to control speed (forward = fast, back = slow)
		- Ability to control the angle (up = up, down = down)
	- A second hitbox after making contact (bomb?)
	- Ability to pick them back up / restock
	- Air version?

# Item ideas
## Blade oil
- Consumable
- Retains [#Sharpness](/#Sharpness) from the previous round

## Smithy coupon
- Consumable
- Add [#Sharpness](/#Sharpness) on round start