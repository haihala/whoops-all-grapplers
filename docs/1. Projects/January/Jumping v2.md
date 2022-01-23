- [x] Prejump frames
	- [x] Add prejump state (ground)
- [x] Short and long hops
	- [x] If holding up at the end of prejump, you get a long hop
- [ ] Investigate double hops while at it
	- Likely are a product of parallelization
	- They definitely still happen. Even with the prejump, for some reason the event is sent twice.
- [x] Why does doubling the impulse give so much more height?
- [x] Why does it always long jump

Probably one function in `movement.rs`, state can determine internally if this is a jump event or a prejump prep.