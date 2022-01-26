Separated so that `player_state` can depend on `input_parsing`  and not the other way. Put the new flipped into types.

AbsoluteDirection -> (LRD)irection (Left - Right direction). That handles flipping everything internally.