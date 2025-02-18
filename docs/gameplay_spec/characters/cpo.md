# CPO

Chief Pounding Officer, a robotic Vince McMahoon

## Design question block

This is for the things that need to be thought of

- Is [Black flash](#black-flash) an annoying execution check?
  - Yes, you have no reason to not go for it
  - Maybe if it costs bar?
    - What does the button do without bar?
  - Maybe if there is a different cost on whiff?
    - Go down a level
    - Forced stumble animation
- How about them fever levels
  - Options:
    - Keep 4 as ad break, put a parry on 3
    - Move ad break on 3, make 4 an instant kill
- Timewinder sweet spots
  - Ideas for properties
    - Speed increase
    - Damage increase
    - Launch
    - Wall bounce
    - Huge hitstop
      - Stops their time, allows a hit after
      - Clock vfx + ticking sfx
    - [Fever](#fever) level
- Maybe reflavor [Ad break](#ad-break)?

## Lore

- The owner of [W.A.G.](/docs/gameplay_spec/lore/w.a.g..md) in kayfabe.
- Clearly a heel

## Motifs

- Vibes
  - His character is
    - THE boss
    - Huge and intimidating
    - Larger than life
  - "Fever" as explained by Hakari
- Aesthetics
  - Massive man in a pinstripe suit
  - Television for a head that displays text occasionally
  - Gold watch in both hands
  - G-like, theatric mannerisms

## Place in the roster

- Technical bruiser
  - Bruisers tend to be a bit braindead
    - I find this interesting
- The boss
- Window to the business

## Mechanics

### Fever

- Fever has a level and a timer
- Fever level gives a damage and speed multiplier
- New moves get unlocked at each level
  - Start at level 0 with just [Sugarcoat](#sugarcoat)
  - At level 1: [Timewinder](#timewinder)
  - At level 2: [Pay check](#pay-check)
  - At level 3:
  - Finally at level 4: [Ad break](#ad-break)
- Fever level goes down when:
  - Fever timer finishes
  - CPO gets hit
- Meant to go up and down a lot
  - You can go from 0 to 4 within one combo
  - You can go back down to 0 by getting reset a few times
- Getting to the final level is supposed to be very tricky
- Visually, he gains more effects at higher levels
  - The monitor on his head changes
  - Aura (shader on a cylinder, world position coords)

### Black flash

- Input: `g` in any state besides hitstun
- Activating starts a cooldown of about 2s
- If a strike lands in the next few frames
  - [Fever](#fever) level increases and timer resets
  - Cooldown resets
  - The activating move gains improved properties

## Moves

### Normals

input, visual, function

#### Credit card swipe

- Input: `5f`
- Visual: Gief credit card swipe
- Function: Anti-air, poke
- Can be ducked
- Not that fast vs grounded (Hitbox descends)

#### Dick jab

`2f` option based on how it goes with [Credit card swipe](#credit-card-swipe)

- Input: `2f`
- Visual: Gordeau 2A, maybe G cr.lp
- Function: Fastest move, neutral poke

#### Jumping knees

- Input: `j.f`
- Visual: Gief SF6 j.lk
- Function: Quick air to air, fuzzy tool with something that gets you airborne

#### Hook punch

- Input: `5s`
- Visual: Sol c.s / f.s hybrid
- Function: Launcher, combo tool, big damage starter

#### Stomps

- Input: `2s[2s[2s]]`
- Visual: Gief SF6 2mk series
- Function: Low, grounded combo damage, build up time to charge

#### Overhead

- Input: `6s`
- Visual: Double-fisted overhead swing
- Function: Standing overhead
- Input has intentional implications with [Timewinder](#timewinder) charge

#### Body splash

- Input: `j.s`
- Visual: Any grappler body splash
- Function: Ambiguous jump-in, somwhat tricky to anti-air

#### Ground throw

- Input: `5w`
- Visual: G / Bane knee to spine
- Function: Ground throw
- Back throw input as usual
  - Animation forks when the knee hits the back

#### Air throw

- Input: `j.w`
- Visual: Android 16 air grab
- Function: Air to air, juggle ender
- Has a lot of extra landing lag on whiff
- Implement a hard knockdown system for this

### Specials

#### Sugarcoat

- Input: `236g`
- Visual: Honda rice throw (SF walk-in) / MvC2 magneto 5hp
  - https://wiki.supercombo.gg/w/Marvel_vs_Capcom_2/Magneto
- Will cover opponent with sugar on hit or block
- Very active
- When covered in sugar, next hit will auto-trigger [Black flash](#black-flash)
- Sugar status lasts for about 5s or until CPO is hit

#### Timewinder

- Visual: Balrog dash punch / Sol sidewinder
  - Has a small hitbox sweet spot on hit
- Input:
  - Charge: `[14]`
    - Smooth scale charge
      - You can do it with 0 charge
      - 99% of charge is a sweet spot
  - Flick:
    - Ground:
      - `6`: Hits mid can link after on hit
      - `3`: Hits low, knocks down, but worse on block
    - Air: `[963]`, juggle tool, hits mid, can "TK"
  - Press:
    - `f`: Short and fast
      - Can be spaced to be safe
      - Use cases:
        - Spacing trap pressure tool
        - Light combo special cancel
    - `s`: Powerful and longer lunging
      - Punishable
      - Use case:
        - Combo tool
        - Unsafe jumpscare
    - `(fs)`: Fastest and launching
      - Death on block / whiff
      - Use case: Giga punish starter
- So in total you have:
  - 3 Directional versions (air, mid, low)
  - 3 Button versions (`f`, `s`, `(fs)`)
  - 8 Permutations of sweet spots (2^(space, charge, black flash))
  - So a total of 72 versions

#### Pay check

- Input: `236w`
- Visual: Jamie command grab, but holds a fistful of bank notes on hit
- On hit:
  - Deals no damage
  - Takes some of their money
  - Leaves them staggered so you can combo
- Will have them acting, like they won't get command grabbed

#### Ad break

- Input: `214g`
- Activates an install
  - Aesthetically similar to Hakari's domain expansion
    - Does the hand sign
    - Changes the music
- While active
  - Generates about 1/3 segments of meter per second
  - If meter is full, health regenerates about 20/s
  - [Fever](#fever) timer is notably slower
- Consumes all full meter segments
  - Duration increases exponentially per segment
  - Lasts something like 2^(segments-1) seconds
    - 0 bars: 1s -> 1/3 bar
    - 1 bars: 2s -> 2/3 bars
    - 2 bars: 4s -> 1 + 1/3 bars
    - 3 bars: 8s -> 2 + 2/3 bars (almost break even)
    - All 4 bars: 16s -> 5 + 1/3 bars
- If hit, ends the effect
- Items
  - Passive income when active
  - Hits reduce duration, don't end it outright

## Ideas

- Timer manipulation
  - Directional 360 input
    - clockwise moves clock forward
- Game speed manipulation
  - Personal?
  - Speeding up costs a resource that you gain by slowing down
- Tracer rewind
