# CPO

Chief Pounding Officer, a robotic Vince McMahoon

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

### Jackpot

- `g` in any state besides hitstun activates timer
  - Full bar is 2s
    - Timer is also indicated as a ring that contracts around the character
    - At the end the ring expands to a thin pillar and vanishes after time is up
  - Pressing it again stops it
    - Things grants a 1s buff
      - Buff level depends on how close to the 1.5s mark you got
        - There is an bell that indicates level
          - N dings for level N
            - Accompanied by a ripple effect on the ground
          - Failure sound for misses
          - Dismissal sound for untriggered
        - Visual effect as well
          - Super Sayan aura flashing up and down
    - If buff is unused, forces chest grabbing animation
  - If not stopped, nothing happens
    - There is a 1s cooldown before timer can start again
- Buff levels
  - Level 3
    - Frame perfect (1f)
    - Buff effects
      - +100% damage
      - +20f hitstop
      - Gain a segment of bar
  - Level 2
    - Window is +-2f (5f)
    - Reasonably achievable success
    - Buff effects are half of level 3
  - Level 1
    - Window is +-14f (29f)
    - Borderline success
    - Buff effects are half of level 2

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
- When covered in sugar, next [Jackpot](#Jackpot) hit will upgrade by one degree
  - Allows for level 4, which has double the effects of level 3
- Sugar status lasts for about 5s or until CPO is hit

#### Timewinder

- Visual: Balrog dash punch / Sol sidewinder
  - First does a shoulder bash
    - Knocks target airborne
      - Sometimes unwanted
- Input:
  - Charge: `[14]`
    - Smooth scale charge
      - You can do it with 10% charge
        - Prevent accidentals
      - More charge goes further
  - Flick:
    - Ground:
      - `6`: Hits mid can link after on hit
      - `3`: Hits low, knocks down, but worse on block
    - Air: `[963]`, juggle tool, hits overhead, can "TK"
  - Press:
    - `f`: Short and fast
      - Can be spaced to be safe on block
      - Use cases:
        - Spacing trap pressure tool
        - Light combo special cancel
    - `s`: Powerful and longer lunging
      - Punishable on block
      - More upwards knock on shoulder
      - Use case:
        - Combo tool
        - Unsafe jumpscare
    - `(fs)`: Fastest longest reach
      - Notably longer lunge before the shoulder
      - Wall bounces
      - Death on block / whiff
      - Use case: Giga punish starter
- What do the sweet spots do
  - [Jackpot](#jackpot) explains itself
  - Clean hit shoulder
    - Knocks airborne
      - There is a loop that depends on NOT hitting it
      - There is a combo that depends on exclusively hitting it
  - Charge
    - Lunge distance
      - 99% charge has the most distance
      - 100% charge has like 75% of 99% charge lunge
    - Maybe there is not a specific "sweet spot", but it depends on use case

#### Pay check

- Input: `236w`
- Visual: Jamie command grab, but holds a fistful of bank notes on hit
- On hit:
  - Deals no damage
  - Takes some of their money
  - Leaves them staggered so you can combo
- Will have them acting, like they won't get command grabbed

#### Ad break

- Input: `214g[4g[4g[4g]]]`
  - Four degrees of commitment, each lengthening the animation a bit
- Activates an install
  - Aesthetically similar to Hakari's domain expansion
    - Does the dance, length depends on input
    - Changes the music while active
- While active
  - Generates about 1/3 segments of meter per second
  - If meter is full, health regenerates about 20/s
    - Editor note: This may be too much
      - Although if you have max bar you deserve it
  - 200% width [Jackpot](#jackpot) windows (except for Level 3 and 4)
- Duration increases exponentially per button in input
  - 1 press: 3s -> 1 bar / 60 health
  - 2 presses: 6s -> 2 bars / 120 health
  - 3 presses: 9s -> 3 bars / 180 health
  - All 4 presses: 12s -> 4 bars / 240 health
- Effect ends on hit
- Items
  - Passive income when active, extra if health is full
  - Hits reduce duration, don't end it outright

## Ideas

- Timer manipulation
  - Directional 360 input
    - clockwise moves clock forward
- Game speed manipulation
  - Personal?
  - Speeding up costs a resource that you gain by slowing down
- Tracer rewind
- Launcher version to grounded [[#Timewinder]] (end in up+forward?)
  - Overlaps with TK air version
