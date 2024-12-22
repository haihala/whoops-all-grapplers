# Main doc

This is the one stop shop for what's up in the project

## Next steps

### Before contacting artists

- Check for free SFX / VA work

### Before public playtests

- Character 2
  - Animations
  - Materials (pinstripes and skin)
- Tutorial (mostly text)
- Pretty loading screen (Move lists)
- Settings (volume and keybinds)
- [Input parser rework](/docs/tasks/backlog/input_parser_rework.md)
- Pause / Quit while in a match

### Backlog

Low priority do later tasks

- [Emote button](/docs/tasks/backlog/emote_button.md)
- [Rewards 2.0](/docs/tasks/backlog/rewards_2.0.md)
- [Yoink assets](/docs/tasks/backlog/yoink_assets.md)
- [Netplay 2.0](/docs/tasks/backlog/netplay_2.0.md)
- Delay wakeup on knockdown
- [Popups 3.0](/docs/tasks/backlog/popups_3.0.md)
- Pushback should not apply to projectiles
- Stackable items
  - Make thumbtacks a single item you can buy multiple times
    - It takes up too much space in the menu
  - Add a similar item for defense
- Experiment with juggle combos
  - Add tools to enable configuring air hit properties.
  - Maybe ice cube default is the thing?
  - Maybe less knockback on air hits because lesser friction?

## Overarching plan

- [x] MVP (Not a rushed one, but one that is maintainable)
- [x] Feedback round 1
- [x] Polish
- [x] Test with fgc grinders
- [x] Backburner until DI is done
- [ ] Start building the community
- [ ] Release to early access
- [ ] Adjust based on stranger feedback until I can't afford bread

## Bug terrarium

### Known issues

- Network bugs
  - Sometimes synctest spawns with 0 kunais
  - Immediate desync on some starts
    - Seems like a system ordering inconsistency
  - Ghost objects

### Under review

- Defender can't block 2f spam and there are opener sparks (had paint)
- Stick input occasionally gets stuck
  - Replicate with 46464646... on the dpad
