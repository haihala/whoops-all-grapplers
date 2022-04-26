https://bevyengine.org/news/bevy-0-7
https://bevyengine.org/learn/book/migration-guides/0.6-0.7/

- Change versions in `cargo.toml`s
- [System ordering](https://bevyengine.org/news/bevy-0-7/#ergonomic-system-ordering) (auto labels)
- `..Default::default()` -> `..default()` (in prelude)
- [Accessing many items in a query](https://bevyengine.org/news/bevy-0-7/#query-many)
	- Think about using a global lookup resource for players instead of having a player component on the entities like in the first example
- [See if we can use the newtype pattern more](https://bevyengine.org/news/bevy-0-7/#deref-derefmut-derives)
- [Custom world queryies](https://bevyengine.org/news/bevy-0-7/#worldquery-derives)
	- Collision and move advancement come to mind
- [World::get_resource -> World::resource](https://bevyengine.org/news/bevy-0-7/#world-resource)
	- Old variant is still there, but new doesn't need unwrapping
- [Move player sprite anchor](https://bevyengine.org/news/bevy-0-7/#sprite-anchors)
	- To middle bottom so that crouching doesn't need a custom solution