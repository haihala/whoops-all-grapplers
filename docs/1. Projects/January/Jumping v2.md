- [ ] Prejump frames (5+)
	- [ ] Add prejump state (air)
- [ ] Short and long hops
	- [ ] If holding up at the end of prejump, you get a long hop
- [ ] Investigate double hops while at it
- [ ] Likely has to do with parallelization

Probably one function in `movement.rs`, state can determine internally if this is a jump event or a prejump prep.