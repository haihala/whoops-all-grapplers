Blocked by [[Player state refactor]]

- Input repetition out of parsing and into new move starter system
- This allows judging options based on state (don't blindly repeat but lock in options when you can't immediately execute.)
- This allows you to more easily see if the user is mashing or not
- Add a buffer component that reads in incoming events and then starts them whenever state lets you