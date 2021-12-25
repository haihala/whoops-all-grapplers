Tasks:
- [x] Separate input parsing from input event generation
- [x] Motion parsing to Special impl
- [x] Define requirements as a list of enums:
	- [x] Whitelist of inputs that advance to the next, "basic"
	- [x] Whitelist of inputs and a duration, "hold"
		- [x] The stick position must be one of for that duration
	- [x] Button press, "press"
	- [x] Button release, "release"
- [x] Old stick != new stick assert triggers presumably if event is done and undone within a frame.
- [x] Method to convert `&'static str` to a special move
	- [x] Implementation
	- [x] Tests
	- [x] Ditto for normals, since most of the work has already been done.
- [x] Special move interpretation advancement rewrite
	- [x] Multipress


Bug was caused by not recursively resolving charges