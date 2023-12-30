This is a more generic way to pass information to the situational deciders.

- Currently, if a move has hit the opponent, a flag is stored in the history
- If a move hits an invulnerable foe, a flag is stored on the hitbox's hit tracker
- Would be cool of both of these followed a unified channel.
	- Add an `ExternalEvent` enum
	- Add a `vec<ExternalEvent>` to history with helper functions