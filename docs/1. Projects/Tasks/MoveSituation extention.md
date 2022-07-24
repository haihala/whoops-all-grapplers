#blocked by [[Move phase format rework]]
 
 - `MoveSituation` ought to include the phase resolution history so that it becomes deterministic
	- Currently it can't handle cases where multiple phases depend on input
	- Re-evaluating for later ones includes the input for previous ones, which causes issues
- Consider the fields in light of [[Move phase format rework]]
	- For example, cost doesn't really make sense
	- Could you just include the move id of the previous move and a history representation for most of the current fields?
- `MoveSituation` shouldn't be the ultimate source of truth as to where a move is going
	- It's cumbersome to maintain and with the new clojure approach for decision making, unnecessary.
