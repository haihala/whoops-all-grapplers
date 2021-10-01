# Special move interpretation
- [x] Store latest processed frame
- [x] If latest processed frame is different from latest frame, check all incomplete and add to them if it matches
- [x] Store partial special moves
	- What move
	- When does it start
	- What is the next requirement
	- A method for "This input came in, are we ready yet?"
- [x] Clear stored special moves when first frame leaves the buffer
	- [x] Check if input has restarted at some point
- [x] Instead of a hard buffer size, have a value of time per input in buffer
	- This would make buster and quarter circle input not stay in the buffer for as long
	- The problem with this is if you crouch block and move forwards, a quarter circle is in the buffer for a second.  Now if you press a button, it'll always interpret it as a quarter, because there is a quarter in the last second of inputs 

# Recent buffer incremental updates
- [x] Separate recent from event queue
	- Event queue is for intermittent parsing like special moves and recent
- [x] Remove first buffer of short buffer if it's equal to newest illegal frame
	- [x] Remove inputs from recently pressed/released

# Move input parsing to a new crate
Input parsing crate will provide:
- [x] Enum for buttons
- [x] Struct for motion inputs
- [x] Struct for Special moves (motion + button)
- [x] A way to register special moves and receive events when they are fulfilled
- [x] Read current inputs
- [x] A plugin that can parse raw inputs
- [x] Rewrite docs
- [x] Event repeater
