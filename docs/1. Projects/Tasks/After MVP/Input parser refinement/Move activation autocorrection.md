#blocked by [[Sub inputs]]

- Problem is that if the non-ex version starts, nothing is done if the second EX button comes in a frame late.
- Have a minimum startup (5f)
	- That is the window in which a retro correction can happen
- How to tell what autocorrects into what
	- Use [[Sub inputs]], tiebreak with moveId ord
- New system (like linking and canceling) for retroactive corrections
	- Detects if a higher priority move is now in the buffer than the current running one
	- If so and we're within the min startup buffer, replace it.

- Did some testing
	- on keyboard, a 1 frame correction would probably be fine
	- on a pad, some thumb positions lead to inconsistencies
	- Make it a 3 frame difference window
		- if nothing happens on the first three frames than that is a-ok