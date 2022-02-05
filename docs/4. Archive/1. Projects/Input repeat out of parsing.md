Blocked by [[Player state refactor]]

- [x] Input repetition out of parsing and into new move starter system
- [x] This allows judging options based on state (don't blindly repeat but lock in options when you can't immediately execute.)
- [x] This allows you to more easily see if the user is mashing or not
- [x] Add a buffer component that reads in incoming events and then starts them whenever state lets you