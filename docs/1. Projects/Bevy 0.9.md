- Basic migration (it builds)
- Plugin settings (integration tests?)
- [Global time scaling](https://bevyengine.org/learn/book/migration-guides/0.8-0.9/#add-global-time-scaling) for hitstop
- Inputs
	- Investigate [gamepad changes](https://bevyengine.org/learn/book/migration-guides/0.8-0.9/#change-gamepad-rs-tuples-to-normal-structs)
	- [InputAxis](https://bevyengine.org/learn/book/migration-guides/0.8-0.9/#add-getters-and-setters-for-inputaxis-and-buttonsettings) changes
	- Use [gamepadinfo](https://bevyengine.org/learn/book/migration-guides/0.8-0.9/#add-gamepadinfo-expose-gamepad-names) to show connected device name
	- [Gamepad copy](https://bevyengine.org/learn/book/migration-guides/0.8-0.9/#gamepad-type-is-copy-do-not-require-return-references-to-it-in-gamepads-api)
- [System piping](https://bevyengine.org/learn/book/migration-guides/0.8-0.9/#rename-system-chaining-to-system-piping)

# Broken
- UI
	- Meter bar isn't visible
	- Notifications are shown where meter ought to be
	- [Default UI background color is transparent](https://bevyengine.org/learn/book/migration-guides/0.8-0.9/#make-the-default-background-color-of-nodebundle-transparent)
- [Animations play/start](https://bevyengine.org/learn/book/migration-guides/0.8-0.9/#rename-play-to-start-and-add-new-play-method-that-won-t-overwrite-the-existing-animation-if-it-s-already-playing)
