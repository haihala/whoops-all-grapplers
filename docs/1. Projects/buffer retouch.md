# Special move interpretation
- [ ] Store latest processed frame
- [ ] If latest processed frame is different from latest frame, check all incomplete and add to them if it matches
- [ ] Store partial special moves
	- What move
	- When does it start
	- What is the next requirement
	- A method for "This input came in, are we ready yet?"
- [ ] Clear stored special moves when first frame leaves the buffer
	- [ ] Check if input has restarted at some point
- [ ] Interprep backwards
- [ ] Instead of a hard buffer size, have a value of time per input in buffer
	- This would make buster and quarter circle input not stay in the buffer for as long
	- The problem with this is if you crouch block and move forwards, a quarter circle is in the buffer for a second.  Now if you press a button, it'll always interpret it as a quarter, because there is a quarter in the last second of inputs 

# Recent buffer incremental updates
- [ ] Store a smaller 'recent' cloned diff buffer
- [ ] Add new diffs to short buffer
	- [ ] Add inputs to recently pressed/released
- [ ] Calculate newest illegal frame
- [ ] Remove first buffer of short buffer if it's equal to newest illegal frame
	- [ ] Remove inputs from recently pressed/released
