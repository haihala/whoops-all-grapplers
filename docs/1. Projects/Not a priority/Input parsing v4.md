Tasks:
- [x] Separate input parsing from input event generation
- [ ] Motion parsing to Special impl
- [ ] Define requirements as a list of enums:
	- [ ] Whitelist of inputs that advance to the next, "basic"
	- [ ] Whitelist of inputs and a duration, "hold"
		- [ ] The stick position must be one of for that duration
	- [ ] Button press, "press"
	- [ ] Button release, "release"
- [ ] Old stick != new stick assert triggers presumably if event is done and undone within a frame.