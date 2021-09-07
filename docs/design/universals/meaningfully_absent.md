# Meaningfully absent
## Combo scaling
Upsides:
- It emphasizes shorter combos
- It reduces touch of deaths

Downsides:
- Difficult to really understand (did that small hit early mean I did less damage with more hits?)
- Not really visible (sitting new fighting gamers out there I guarantee they don't even notice combo scaling outside training mode numbers)

Workaround:
- Ways to break a combo (Overdrive)
- World of warcraft it. Instead of combo damage tapering off, you have a poke bonus
  - Poke bonus (rename): After not landing an unblocked hit for five seconds, your next hit does extra damage.
  - TODO: Separate file

## Running
Upsides:
- Go fast

Downsides:
- You rarely want to go slow, so you just have to jump through extra hoops.

Workaround:
- Every character is always running.

## Counter hit
Upsides:
- Encourages noobs to use smaller moves
- Rewards knowing your character and the matchup

Downsides:
- Rich get richer
- Often happen by accident

Workaround:
- Try without to go without. If it looks like the game could benefit from counter hits, reconsider

## Block invincibility on getting up
Upsides:
- Simplify oki

Downsides:
- Rage inducing "Why did that not throw ree" -situations

Workaround:
- Just don't bother with it, include throws in the oki rps.

## Hard and soft knockdown
Upsides:
- Balance levers
- Differences in kind

Downsides:
- Complexity with little gain
- Confusing for new players

Workaround:
- Just don't


## Ground invincibility
Upsides:
- Prevents many loops

Downsides:
- Somewhat unintuitive

Workaround:
- Shorten getting up time
- Custom oki logic:
  - No matter what option the defender picks, the attacker should have at best a 50/50 situation in the oki RPS.
  - Attacker options:
    - Meaty strike
    - Meaty grab
    - Block
  - Defender options:
    - Blocking
      - Even with block
      - Beats strike
      - Loses to grab
    - Jump
      - Loses to grab
      - Even with block
      - Beats some strikes
    - Dash
      - Back
        - Even with block
        - Wins grab
        - Loses to some, wins some strikes
      - Forward
        - Even with block
        - Loses to grab
        - Loses to some, wins some strikes
    - Reversal (not all characters)
      - Beats strike and grab
      - Loses to block
    - Break recovery (costs zeal)
      - Recover faster than expected so that you avoid the situation entirely
      - Loses to strikes and grabs, but they need to read it
  - Concocted situation intentionally makes letting the other person get up never really lose.
