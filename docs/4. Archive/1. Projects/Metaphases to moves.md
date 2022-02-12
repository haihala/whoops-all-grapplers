Problem: How to handle canceling on a whiff
- If the move has hit, go into a cancellable phase, if not, fall back to a non-cancellable one.

# New solution
Something like:

```rust
// https://docs.rs/bitflags/latest/bitflags/#
bitflags! {
	struct Condition: u32 {
		const Airborne = 1;
		const Blocking = 2;
		// ...
	}
}


enum MetaPhase {
	Plain(Move),
	Switch(HashMap<Condition, Move>)
}

impl MetaPhase {
	pub fn get(&self, condition: Condition) -> Move {
		match self {
			MetaPhase::Plain(move_data) => move_data,
			MetaPhase::Switch(options) => {
				let filtered: Vec<Move> = options.iter().filter_map(|cond, mv| if cond == condition {Some(mv)} else {None}).collect()

				if filtered.len() == 1 {
					return filtered.get(0).unwrap()
				}

				if filtered.len() == 0 {
					panic!("Didn't find a valid phase");
				} else {
					panic!("Found multiple valid phases")
					// TODO: Somehow find the strictest one here
				} 
			}
		}
	}
}

```
