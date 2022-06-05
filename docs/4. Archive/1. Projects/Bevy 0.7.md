https://bevyengine.org/news/bevy-0-7
https://bevyengine.org/learn/book/migration-guides/0.6-0.7/
- [x] QuerySet and QueryState to ParamSet and Query
- [x] Name out of Camera
- [x] Camera frustrum

- [x] Change versions in `cargo.toml`s
- [x] `..Default::default()` -> `..default()` (in prelude)
- [x] [System ordering](https://bevyengine.org/news/bevy-0-7/#ergonomic-system-ordering) (auto labels)
	- Nearly came after seeing the results of this holy shit it makes stuff neat
- [x] [Accessing many items in a query](https://bevyengine.org/news/bevy-0-7/#query-many)
	- Think about using a global lookup resource for players instead of having a player component on the entities like in the first example
- [x] [See if we can use the newtype pattern more](https://bevyengine.org/news/bevy-0-7/#deref-derefmut-derives)
	- Timers?
	- Owners for hitboxes
- [x] [Custom world queryies](https://bevyengine.org/news/bevy-0-7/#worldquery-derives)
	- Collision and move advancement come to mind
- [x] [World::get_resource -> World::resource](https://bevyengine.org/news/bevy-0-7/#world-resource)
	- Old variant is still there, but new doesn't need unwrapping
- [x] [Move player sprite anchor](https://bevyengine.org/news/bevy-0-7/#sprite-anchors)
	- To middle bottom so that crouching doesn't need a custom solution
	- If air crouching makes it's way into the game, it ought to revert this or maybe set it dynamically